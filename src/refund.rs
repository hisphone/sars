use crate::{
    from_file::{FromExcel, FromFormatedExcel},
    to_excel::{Columns, Header, SheetName, ToExcel, ToRow},
};
use calamine::{open_workbook_auto, DataType, Reader};
use serde::{Deserialize, Serialize};
use simple_excel_writer::{row, Column, Row};
use std::{path::Path, slice::Iter, vec::IntoIter};

#[derive(Debug, Default)]

pub struct Refunds(Vec<Refund>);
impl AsRef<Vec<Refund>> for Refunds {
    fn as_ref(&self) -> &Vec<Refund> {
        &self.0
    }
}

impl AsMut<Vec<Refund>> for Refunds {
    fn as_mut(&mut self) -> &mut Vec<Refund> {
        &mut self.0
    }
}

impl<P> FromExcel<P> for Refunds
where
    P: AsRef<Path>,
{
    fn from_excel(path: P) -> anyhow::Result<Self> {
        let mut workbook = open_workbook_auto(path).unwrap();
        let worksheet = workbook
            .worksheet_range_at(0)
            .ok_or(anyhow::Error::msg("range err"))??;
        let mut refunds = Refunds::default();
        let mut temp = "".to_string();
        for row in worksheet
            .rows()
            .skip(4)
            .map(|row| (&row[4], &row[40], &row[41], &row[44]))
        {
            match row {
                (
                    DataType::String(date),
                    DataType::String(id),
                    DataType::String(name),
                    DataType::String(refund),
                ) => {
                    temp = date.to_string();
                    let refund = Refund::new(
                        ("102831264647".to_string() + &id).parse().unwrap(),
                        name.to_string(),
                        temp.to_string(),
                        refund.parse().unwrap(),
                    );
                    refunds.as_mut().push(refund);
                }
                (
                    DataType::Empty,
                    DataType::String(id),
                    DataType::String(name),
                    DataType::String(refund),
                ) => {
                    let refund = Refund::new(
                        ("102831264647".to_string() + &id).parse().unwrap(),
                        name.to_string(),
                        temp.to_string(),
                        refund.parse().unwrap(),
                    );
                    refunds.as_mut().push(refund);
                }
                _ => println!("{row:?}"),
            }
        }

        Ok(refunds)
    }
}
impl<'a> IntoIterator for &'a Refunds {
    type Item = &'a Refund;

    type IntoIter = Iter<'a, Refund>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}
impl IntoIterator for Refunds {
    type Item = Refund;

    type IntoIter = IntoIter<Refund>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> Header for &'a Refunds {
    fn header(&self) -> Row {
        row!["日期", "账户账号", "账户名称", "退款"]
    }
}
impl<'a> SheetName for &'a Refunds {
    fn sheet_name(&self) -> &'static str {
        "退款明细"
    }
}
impl<'a> Columns for &'a Refunds {
    fn column(&self) -> Vec<simple_excel_writer::Column> {
        vec![
            Column { width: 30.0 },
            Column { width: 30.0 },
            Column { width: 30.0 },
            Column { width: 30.0 },
        ]
    }
}
impl<'a> ToExcel for &'a Refunds {}
impl From<Vec<Refund>> for Refunds {
    fn from(v: Vec<Refund>) -> Self {
        Self(v)
    }
}
impl<P: std::convert::AsRef<std::path::Path>> FromFormatedExcel<'_, P> for Refunds {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Refund {
    #[serde(rename = "账户账号")]
    #[serde(deserialize_with = "de_opt_u64")]
    id: u64,
    #[serde(rename = "账户名称")]
    name: String,
    #[serde(rename = "日期")]
    date: String,
    #[serde(rename = "退款")]
    refund: f64,
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

impl Refund {
    pub fn new(id: u64, name: String, date: String, refund: f64) -> Self {
        Self {
            id,
            name,
            date,
            refund,
        }
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_date(&self) -> &String {
        &self.date
    }
    pub fn get_refund(&self) -> f64 {
        self.refund
    }
}
impl<'a> ToRow for &'a Refund {
    fn to_row(&self) -> Row {
        row![
            self.date.as_str(),
            self.id.to_string(),
            self.name.as_str(),
            self.refund
        ]
    }
}
