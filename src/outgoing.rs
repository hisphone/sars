use crate::{
    from_file::{FromExcel, FromFormatedExcel},
    to_excel::{Columns, Header, SheetName, ToExcel, ToRow},
};
use calamine::{open_workbook_auto, DataType, Reader};
use serde::{Deserialize, Serialize};
use simple_excel_writer::{row, Column, Row};
use std::{path::Path, slice::Iter, vec::IntoIter};

#[derive(Debug, Default)]
pub struct Outgoings(Vec<Outgoing>);

impl AsRef<Vec<Outgoing>> for Outgoings {
    fn as_ref(&self) -> &Vec<Outgoing> {
        &self.0
    }
}

impl AsMut<Vec<Outgoing>> for Outgoings {
    fn as_mut(&mut self) -> &mut Vec<Outgoing> {
        &mut self.0
    }
}

impl<P> FromExcel<P> for Outgoings
where
    P: AsRef<Path>,
{
    fn from_excel(path: P) -> anyhow::Result<Self> {
        let mut workbook = open_workbook_auto(path).unwrap();
        let worksheet = workbook
            .worksheet_range_at(0)
            .ok_or(anyhow::Error::msg("range err"))??;
        let mut outgoings = Outgoings::default();
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
                    DataType::String(outgoing),
                ) => {
                    temp = date.to_string();
                    let outgoing = Outgoing::new(
                        ("102831264647".to_string() + &id).parse().unwrap(),
                        name.to_string(),
                        temp.to_string(),
                        outgoing.parse().unwrap(),
                    );
                    outgoings.as_mut().push(outgoing);
                }
                (
                    DataType::Empty,
                    DataType::String(id),
                    DataType::String(name),
                    DataType::String(outgoing),
                ) => {
                    let outgoing = Outgoing::new(
                        ("102831264647".to_string() + &id).parse().unwrap(),
                        name.to_string(),
                        temp.to_string(),
                        outgoing.parse().unwrap(),
                    );
                    outgoings.as_mut().push(outgoing);
                }
                _ => println!("{row:?}"),
            }
        }

        Ok(outgoings)
    }
}
impl<'a> IntoIterator for &'a Outgoings {
    type Item = &'a Outgoing;

    type IntoIter = Iter<'a, Outgoing>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}
impl IntoIterator for Outgoings {
    type Item = Outgoing;

    type IntoIter = IntoIter<Outgoing>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> Header for &'a Outgoings {
    fn header(&self) -> Row {
        row!["日期", "账户账号", "账户名称", "支出"]
    }
}
impl<'a> SheetName for &'a Outgoings {
    fn sheet_name(&self) -> &'static str {
        "支出明细"
    }
}
impl<'a> Columns for &'a Outgoings {
    fn column(&self) -> Vec<simple_excel_writer::Column> {
        vec![
            Column { width: 30.0 },
            Column { width: 30.0 },
            Column { width: 30.0 },
            Column { width: 30.0 },
        ]
    }
}
impl<'a> ToExcel for &'a Outgoings {}
impl From<Vec<Outgoing>> for Outgoings {
    fn from(v: Vec<Outgoing>) -> Self {
        Self(v)
    }
}
impl<P: std::convert::AsRef<std::path::Path>> FromFormatedExcel<'_, P> for Outgoings {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Outgoing {
    #[serde(rename = "账户账号")]
    #[serde(deserialize_with = "de_opt_u64")]
    id: u64,
    #[serde(rename = "账户名称")]
    name: String,
    #[serde(rename = "日期")]
    date: String,
    #[serde(rename = "支出")]
    outgoing: f64,
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

impl Outgoing {
    pub fn new(id: u64, name: String, date: String, outgoing: f64) -> Self {
        Self {
            id,
            name,
            date,
            outgoing,
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
    pub fn get_outgoing(&self) -> f64 {
        self.outgoing
    }
}
impl<'a> ToRow for &'a Outgoing {
    fn to_row(&self) -> Row {
        row![
            self.date.as_ref(),
            self.id.to_string(),
            self.name.as_ref(),
            self.outgoing
        ]
    }
}
