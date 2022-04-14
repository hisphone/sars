use std::{
    cell::{Ref, RefCell},
    path::Path,
};

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, ToCellDeserializer, Xls};
use serde::{Deserialize, Serialize};

use crate::{
    balance::Balances, incoming::Incomings, outgoing::Outgoings, refund::Refunds, start::Starts,
};

pub struct SubAccount {
    info: RefCell<SubInfos>,
    start: RefCell<Starts>,
    incoming: RefCell<Incomings>,
    outgoing: RefCell<Outgoings>,
    refund: RefCell<Refunds>,
    balance: RefCell<Balances>,
}

impl SubAccount {
    pub fn new() -> Self {
        Self {
            info: RefCell::new(SubInfos::default()),
            start: RefCell::new(Starts::default()),
            incoming: RefCell::new(Incomings::default()),
            outgoing: RefCell::new(Outgoings::default()),
            refund: RefCell::new(Refunds::default()),
            balance: RefCell::new(Balances::default()),
        }
    }
    pub fn get_info(&self) -> Ref<SubInfos> {
        self.info.borrow()
    }
    pub fn get_start(&self) -> Ref<Starts> {
        self.start.borrow()
    }
    pub fn get_incoming(&self) -> Ref<Incomings> {
        self.incoming.borrow()
    }
    pub fn get_outgoing(&self) -> Ref<Outgoings> {
        self.outgoing.borrow()
    }
    pub fn get_refund(&self) -> Ref<Refunds> {
        self.refund.borrow()
    }
    pub fn get_balance(&self) -> Ref<Balances> {
        self.balance.borrow()
    }
}
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SubInfo {
    #[serde(rename = "单位编码")]
    #[serde(deserialize_with = "de_opt_u32")]
    pub id: u32,
    #[serde(rename = "虚拟子账户名称")]
    name: String,
    #[serde(rename = "VA")]
    va: String,
}
fn de_opt_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let data_type = calamine::DataType::deserialize(deserializer);
    match data_type {
        Ok(calamine::DataType::Float(f)) => Ok(f as u32),
        Ok(calamine::DataType::Int(i)) => Ok(i as u32),
        Ok(calamine::DataType::String(s)) => Ok(s.parse().unwrap_or(0)),
        _ => Ok(0),
    }
}

impl SubInfos {
    pub fn from_excel<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut workbook: Xls<_> = open_workbook(path).unwrap();
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
