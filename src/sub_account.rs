use std::path::Path;

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xls};
use serde::{Deserialize, Serialize};

use crate::{
    balance::Balances, from_file::FromExcel, incoming::Incomings, outgoing::Outgoings,
    refund::Refunds, start::Starts,
};

#[derive(Debug)]
pub struct SubAccount {
    info: SubInfos,
    start: Starts,
    incoming: Incomings,
    outgoing: Outgoings,
    refund: Refunds,
    balance: Balances,
}

impl SubAccount {
    pub fn new(
        info: SubInfos,
        start: Starts,
        incoming: Incomings,
        outgoing: Outgoings,
        refund: Refunds,
        balance: Balances,
    ) -> Self {
        Self {
            info,
            start,
            incoming,
            outgoing,
            refund,
            balance,
        }
    }
    pub fn get_info(&self) -> &SubInfos {
        &self.info
    }
    pub fn get_start(&self) -> &Starts {
        &self.start
    }
    pub fn get_incoming(&self) -> &Incomings {
        &self.incoming
    }
    pub fn get_outgoing(&self) -> &Outgoings {
        &self.outgoing
    }
    pub fn get_refund(&self) -> &Refunds {
        &self.refund
    }
    pub fn get_balance(&self) -> &Balances {
        &self.balance
    }
    pub fn set_info(&mut self, subinfos: SubInfos) {
        self.info = subinfos
    }
    pub fn set_start(&mut self, start: Starts) {
        self.start = start
    }
    pub fn set_incoming(&mut self, incoming: Incomings) {
        self.incoming = incoming
    }
    pub fn set_outgoing(&mut self, outgoing: Outgoings) {
        self.outgoing = outgoing
    }
    pub fn set_refund(&mut self, refund: Refunds) {
        self.refund = refund
    }
    pub fn set_balance(&mut self, balance: Balances) {
        self.balance = balance
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

impl<P> FromExcel<P> for SubInfos
where
    P: AsRef<Path>,
{
    fn from_excel(path: P) -> anyhow::Result<Self> {
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
