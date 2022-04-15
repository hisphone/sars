use std::{
    convert::Infallible,
    path::Path,
    slice::{Iter, IterMut},
    str::FromStr,
};

use simple_excel_writer::{row, Row};

use crate::{
    from_file::{FromTxt, TxtReader},
    to_excel::{Header, IntoExcel, Title, ToRow},
};

#[derive(Debug, Default)]
pub struct Balances(Vec<Balance>);

impl<'a> Header for &'a Balances {
    fn header(&self) -> Row {
        row!["id", "虚拟账户名称", "自有额度"]
    }
}
impl<'a> Title for &'a Balances {
    fn title(&self) -> &'static str {
        "余额"
    }
}
impl<'a> IntoExcel for &'a Balances {}

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
impl<'a> IntoIterator for &'a mut Balances {
    type Item = &'a mut Balance;

    type IntoIter = IterMut<'a, Balance>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().iter_mut()
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
            | "  日期：20220228       币种：人民币                                     第30页/共30页"
            | "  日期：20220228       币种：人民币                                     第29页/共30页"
            | "  日期：20220228       币种：人民币                                     第28页/共30页"
            | "  日期：20220228       币种：人民币                                     第27页/共30页"
            | "  日期：20220228       币种：人民币                                     第26页/共30页"
            | "  日期：20220228       币种：人民币                                     第25页/共30页"
            | "  日期：20220228       币种：人民币                                     第24页/共30页"
            | "  日期：20220228       币种：人民币                                     第23页/共30页"
            | "  日期：20220228       币种：人民币                                     第22页/共30页"
            | "  日期：20220228       币种：人民币                                     第21页/共30页"
            | "  日期：20220228       币种：人民币                                     第20页/共30页"
            | "  日期：20220228       币种：人民币                                     第19页/共30页"
            | "  日期：20220228       币种：人民币                                     第18页/共30页"
            | "  日期：20220228       币种：人民币                                     第17页/共30页"
            | "  日期：20220228       币种：人民币                                     第16页/共30页"
            | "  日期：20220228       币种：人民币                                     第15页/共30页"
            | "  日期：20220228       币种：人民币                                     第14页/共30页"
            | "  日期：20220228       币种：人民币                                     第13页/共30页"
            | "  日期：20220228       币种：人民币                                     第12页/共30页"
            | "  日期：20220228       币种：人民币                                     第11页/共30页"
            | "  日期：20220228       币种：人民币                                     第10页/共30页"
            | "  日期：20220228       币种：人民币                                     第9页/共30页"
            | "  日期：20220228       币种：人民币                                     第8页/共30页"
            | "  日期：20220228       币种：人民币                                     第7页/共30页"
            | "  日期：20220228       币种：人民币                                     第6页/共30页"
            | "  日期：20220228       币种：人民币                                     第5页/共30页"
            | "  日期：20220228       币种：人民币                                     第4页/共30页"
            | "  日期：20220228       币种：人民币                                     第3页/共30页"
            | "  日期：20220228       币种：人民币                                     第2页/共30页"
            | "  日期：20220228       币种：人民币                                     第1页/共30页"
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
            _ => {
                let lines = line.split("│").skip(2).map(|s|s.trim()).collect::<Vec<_>>();
                if lines[1].is_empty() && lines[2].is_empty() {
                    temp = lines[0].to_string()
                } else if temp.trim().is_empty() {
                    balances[index].set_id(lines[0].strip_prefix("102831264647").unwrap().parse().unwrap());
                    balances[index].set_name(lines[1].to_string());
                    balances[index].set_ammount(lines[2].parse().unwrap());
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

#[derive(Debug, Default, Clone)]
pub struct Balance {
    pub id: u32,
    name: String,
    ammount: f64,
}
impl Balance {
    pub fn set_id(&mut self, setter: u32) {
        self.id = setter
    }
    pub fn set_name(&mut self, setter: String) {
        self.name = setter
    }
    pub fn set_ammount(&mut self, setter: f64) {
        self.ammount = setter
    }
}
impl<'a> ToRow for &'a Balance {
    fn to_row(&self) -> Row {
        row![self.id as f64, self.name.as_ref(), self.ammount]
    }
}
