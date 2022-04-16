use crate::{
    from_file::{FromExcel, FromFormatedExcel},
    to_excel::{Columns, Header, SheetName, ToExcel, ToRow},
};
use calamine::{open_workbook_auto, DataType, Reader};
use serde::{Deserialize, Serialize};
use simple_excel_writer::{row, Column, Row};
use std::{path::Path, slice::Iter, vec::IntoIter};

#[derive(Debug, Default)]
pub struct Starts(Vec<Start>);
impl AsRef<Vec<Start>> for Starts {
    fn as_ref(&self) -> &Vec<Start> {
        &self.0
    }
}

impl AsMut<Vec<Start>> for Starts {
    fn as_mut(&mut self) -> &mut Vec<Start> {
        &mut self.0
    }
}
impl<P> FromExcel<P> for Starts
where
    P: AsRef<Path>,
{
    fn from_excel(path: P) -> anyhow::Result<Self> {
        let mut workbook = open_workbook_auto(path).unwrap();
        let worksheet = workbook
            .worksheet_range_at(0)
            .ok_or(anyhow::Error::msg("range err"))??;
        let mut starts = Starts::default();
        for row in worksheet
            .rows()
            .skip(5)
            .map(|row| (&row[0], &row[1], &row[5]))
        {
            match row {
                (DataType::String(id), DataType::String(name), DataType::Float(start)) => {
                    if id.chars().count() == 6 {
                        let start = Start::new(
                            ("102831264647".to_string() + &id).parse().unwrap(),
                            name.to_string(),
                            *start,
                        );
                        starts.as_mut().push(start);
                    }
                }
                (DataType::String(id), DataType::String(name), DataType::Empty) => {
                    if id.chars().count() == 6 {
                        let start = Start::new(
                            ("102831264647".to_string() + &id).parse().unwrap(),
                            name.to_string(),
                            0.0,
                        );
                        starts.as_mut().push(start);
                    }
                }
                _ => println!("{row:?}"),
            }
        }

        Ok(starts)
    }
}
impl<'a> IntoIterator for &'a Starts {
    type Item = &'a Start;

    type IntoIter = Iter<'a, Start>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}
impl IntoIterator for Starts {
    type Item = Start;

    type IntoIter = IntoIter<Start>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> Header for &'a Starts {
    fn header(&self) -> Row {
        row!["账户账号", "账户名称", "年初余额"]
    }
}
impl<'a> SheetName for &'a Starts {
    fn sheet_name(&self) -> &'static str {
        "年初余额"
    }
}
impl<'a> Columns for &'a Starts {
    fn column(&self) -> Vec<simple_excel_writer::Column> {
        vec![
            Column { width: 30.0 },
            Column { width: 30.0 },
            Column { width: 30.0 },
            Column { width: 30.0 },
        ]
    }
}
impl<'a> ToExcel for &'a Starts {}
impl From<Vec<Start>> for Starts {
    fn from(v: Vec<Start>) -> Self {
        Self(v)
    }
}
impl<P: std::convert::AsRef<std::path::Path>> FromFormatedExcel<'_, P> for Starts {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Start {
    #[serde(rename = "账户账号")]
    #[serde(deserialize_with = "de_opt_u64")]
    id: u64,
    #[serde(rename = "账户名称")]
    name: String,
    #[serde(rename = "年初余额")]
    start: f64,
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
impl Start {
    pub fn new(id: u64, name: String, start: f64) -> Self {
        Self { id, name, start }
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_start(&self) -> f64 {
        self.start
    }
}
impl<'a> ToRow for &'a Start {
    fn to_row(&self) -> Row {
        row![self.id.to_string(), self.name.as_ref(), self.start]
    }
}
