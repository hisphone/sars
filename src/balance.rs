use std::{convert::Infallible, path::Path, slice::Iter, str::FromStr, vec::IntoIter};

use serde::{Deserialize, Serialize};
use simple_excel_writer::{row, Column, Row};

use crate::{
    from_file::{FromFormatedExcel, FromTxt, TxtReader},
    to_excel::{Columns, Header, SheetName, ToExcel, ToRow},
};

#[derive(Debug, Default)]
pub struct Balances(Vec<Balance>);

impl<'a> Header for &'a Balances {
    fn header(&self) -> Row {
        row!["账户账号", "账户名称", "实际余额"]
    }
}
impl<'a> SheetName for &'a Balances {
    fn sheet_name(&self) -> &'static str {
        "子账号余额"
    }
}
impl<'a> Columns for &'a Balances {
    fn column(&self) -> Vec<Column> {
        vec![
            Column { width: 30.0 },
            Column { width: 30.0 },
            Column { width: 30.0 },
        ]
    }
}
impl<'a> ToExcel for &'a Balances {}

impl AsRef<Vec<Balance>> for Balances {
    fn as_ref(&self) -> &Vec<Balance> {
        &self.0
    }
}

impl AsMut<Vec<Balance>> for Balances {
    fn as_mut(&mut self) -> &mut Vec<Balance> {
        &mut self.0
    }
}

impl<'a> IntoIterator for &'a Balances {
    type Item = &'a Balance;

    type IntoIter = Iter<'a, Balance>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}
impl IntoIterator for Balances {
    type Item = Balance;

    type IntoIter = IntoIter<Balance>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<P> TxtReader<P> for Balances where P: AsRef<Path> {}

impl<P> FromTxt<P> for Balances where P: AsRef<Path> {}

impl FromStr for Balances {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut balances = vec![Balance::default(); 444];
        let mut index = 0;
        let mut temp = String::new();

        s.lines().rev().for_each(|line| match line {
            " └──┴─────────┴────────────────────┴─────────┘" 
            | " │    │                  │                                        │                  │" 
            | " ├──┼─────────┼────────────────────┼─────────┤" 
            | " │序号│     虚拟账号     │             虚拟账户名称               │     自有额度     │"
            | " ┌──┬─────────┬────────────────────┬─────────┐"
            | "  开户行名称：中国银行宝鸡高新广场支行                                              "
            | "  开户行机构号：16220               "
            | "  主账户名称：宝鸡市市级国库集中支付往来资金账户                                    "
            | "  主账户账号：102831264647        "
            | "                          虚拟账户额度报告单"
            | "1"
            | ""
            => {
                ()
            }
            s if s.starts_with("  日期：") && s.ends_with("页") =>(),
            _ => {
                let lines = line.split("│").skip(2).map(|s|s.trim()).collect::<Vec<_>>();
                if lines[1].is_empty() && lines[2].is_empty() {
                    temp = lines[0].to_string()
                } else if temp.trim().is_empty() {
                    balances[index].set_id(lines[0].parse().unwrap());
                    balances[index].set_name(lines[1].to_string());
                    balances[index].set_balance(lines[2].parse().unwrap());
                    index += 1;
                } else {
                    // balances[index].set_id((lines[0].strip_prefix("102831264647").unwrap().to_string() + &temp).parse().unwrap());
                    // balances[index].set_name(lines[1].to_string());
                    // balances[index].set_ammount(lines[2].parse().unwrap());
                    // index += 1;
                    temp.clear();
                }
            },
        });
        Ok(Self(balances))
    }
}
impl From<Vec<Balance>> for Balances {
    fn from(v: Vec<Balance>) -> Self {
        Self(v)
    }
}
impl<P: std::convert::AsRef<std::path::Path>> FromFormatedExcel<'_, P> for Balances {}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Balance {
    #[serde(rename = "账户账号")]
    #[serde(deserialize_with = "de_opt_u64")]
    id: u64,
    #[serde(rename = "账户名称")]
    name: String,
    #[serde(rename = "实际余额")]
    balance: f64,
}
fn de_opt_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let data_type = calamine::DataType::deserialize(deserializer);
    match data_type {
        Ok(calamine::DataType::String(s)) => Ok(s.parse().unwrap()),
        _ => panic!("refund u64 parse error"),
    }
}
impl Balance {
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_balance(&self) -> f64 {
        self.balance
    }
    pub fn set_id(&mut self, setter: u64) {
        self.id = setter
    }
    pub fn set_name(&mut self, setter: String) {
        self.name = setter
    }
    pub fn set_balance(&mut self, setter: f64) {
        self.balance = setter
    }
}
impl<'a> ToRow for &'a Balance {
    fn to_row(&self) -> Row {
        row![self.id.to_string(), self.name.as_ref(), self.balance]
    }
}
