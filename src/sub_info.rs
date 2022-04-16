use std::{path::Path, slice::Iter, vec::IntoIter};

use calamine::{open_workbook_auto, RangeDeserializerBuilder, Reader};
use serde::{Deserialize, Serialize};
use simple_excel_writer::{row, Column, Row};

use crate::{
    from_file::{FromExcel, FromFormatedExcel},
    to_excel::{Columns, Header, SheetName, ToExcel, ToRow},
};

#[derive(Default, Debug)]
pub struct SubInfos(Vec<SubInfo>);

impl AsRef<Vec<SubInfo>> for SubInfos {
    fn as_ref(&self) -> &Vec<SubInfo> {
        &self.0
    }
}

impl AsMut<Vec<SubInfo>> for SubInfos {
    fn as_mut(&mut self) -> &mut Vec<SubInfo> {
        &mut self.0
    }
}

impl<P> FromExcel<P> for SubInfos
where
    P: AsRef<Path>,
{
    fn from_excel(path: P) -> anyhow::Result<Self> {
        let mut workbook = open_workbook_auto(path).unwrap();
        let worksheet = workbook
            .worksheet_range_at(0)
            .ok_or(anyhow::Error::msg("range err"))??;
        let end = worksheet.end().ok_or(anyhow::Error::msg("range err"))?;
        let range = worksheet.range((1, 4), end);
        let iter = RangeDeserializerBuilder::new()
            .has_headers(true)
            .from_range::<_, SubInfo>(&range)?
            .filter_map(|res| res.ok())
            .collect::<Vec<_>>();

        Ok(Self(iter))
    }
}
impl<'a> IntoIterator for &'a SubInfos {
    type Item = &'a SubInfo;

    type IntoIter = Iter<'a, SubInfo>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}
impl IntoIterator for SubInfos {
    type Item = SubInfo;

    type IntoIter = IntoIter<SubInfo>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> Header for &'a SubInfos {
    fn header(&self) -> Row {
        row!["单位编码", "虚拟子账户名称", "VA"]
    }
}
impl<'a> SheetName for &'a SubInfos {
    fn sheet_name(&self) -> &'static str {
        "子账户信息"
    }
}
impl<'a> Columns for &'a SubInfos {
    fn column(&self) -> Vec<simple_excel_writer::Column> {
        vec![
            Column { width: 30.0 },
            Column { width: 30.0 },
            Column { width: 30.0 },
        ]
    }
}
impl<'a> ToExcel for &'a SubInfos {}
impl From<Vec<SubInfo>> for SubInfos {
    fn from(v: Vec<SubInfo>) -> Self {
        Self(v)
    }
}
impl<P: std::convert::AsRef<std::path::Path>> FromFormatedExcel<'_, P> for SubInfos {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SubInfo {
    #[serde(rename = "单位编码")]
    #[serde(deserialize_with = "de_opt_u64")]
    id: u64,
    #[serde(rename = "虚拟子账户名称")]
    name: String,
    #[serde(rename = "VA")]
    va: String,
}
fn de_opt_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let data_type = calamine::DataType::deserialize(deserializer);
    match data_type {
        Ok(calamine::DataType::String(s)) => {
            if s.starts_with("102831264647") {
                Ok(s.parse().unwrap())
            } else {
                Ok(("102831264647".to_string() + &s).parse().unwrap())
            }
        }
        Ok(calamine::DataType::Float(s)) => Ok(("102831264647".to_string() + &s.to_string())
            .parse()
            .unwrap()),
        Ok(calamine::DataType::Int(s)) => Ok(("102831264647".to_string() + &s.to_string())
            .parse()
            .unwrap()),
        _ => Ok(0),
    }
}
impl SubInfo {
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_va(&self) -> &String {
        &self.va
    }
    pub fn set_id(&mut self, id: u64) {
        self.id = id
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
    pub fn set_va(&mut self, va: String) {
        self.va = va
    }
}
impl<'a> ToRow for &'a SubInfo {
    fn to_row(&self) -> Row {
        row![
            self.get_id().to_string(),
            self.get_name().as_str(),
            self.get_va().as_str()
        ]
    }
}
