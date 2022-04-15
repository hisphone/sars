use std::path::Path;

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xls};
use serde::{Deserialize, Serialize};
use simple_excel_writer::{row, Row, Workbook};

use crate::{
    balance::Balances, from_file::FromExcel, incoming::Incomings, outgoing::Outgoings,
    refund::Refunds, start::Starts, to_excel::ToRow,
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
    pub fn reconcile(&self) {
        let a = self
            .info
            .as_ref()
            .iter()
            .map(|si| {
                let id = &si.id;
                let name = &si.name;
                let start = match self
                    .get_start()
                    .as_ref()
                    .iter()
                    .find(|start| start.get_id() == *id)
                {
                    Some(s) => s.get_start(),
                    None => 0.0,
                };
                let incoming_iter = self
                    .get_incoming()
                    .as_ref()
                    .iter()
                    .filter(|i| i.get_id() == id);
                let incoming = incoming_iter.clone().map(|i| i.get_ammount()).sum::<f64>();
                let incomings = incoming_iter.map(|i| i.to_row()).collect::<Vec<_>>();
                let outgoing_iter = self
                    .get_outgoing()
                    .as_ref()
                    .iter()
                    .filter(|o| o.get_id() == *id);
                let outgoing = outgoing_iter.clone().map(|o| o.get_outgoing()).sum::<f64>();
                let outgoings = outgoing_iter.map(|o| o.to_row()).collect::<Vec<_>>();
                let refund_iter = self
                    .get_refund()
                    .as_ref()
                    .iter()
                    .filter(|r| r.get_id() == *id);
                let refund = refund_iter.clone().map(|r| r.get_refund()).sum::<f64>();
                let refunds = refund_iter.map(|r| r.to_row()).collect::<Vec<_>>();
                let balance = match self
                    .get_balance()
                    .as_ref()
                    .iter()
                    .find(|balance| balance.get_id() == *id)
                {
                    Some(b) => b.get_ammount(),
                    None => 0.0,
                };
                let balance_c = start + incoming - outgoing + refund;

                let mut wb = Workbook::create(
                    std::env::current_exe()
                        .unwrap()
                        .with_file_name(name.clone() + ".xlsx")
                        .to_str()
                        .unwrap(),
                );
                let mut sheet = wb.create_sheet("sheet1");
                wb.write_sheet(&mut sheet, move |sheet_writer| {
                    let sw = sheet_writer;
                    sw.append_row(row![
                        "账号",
                        "账户",
                        "年初余额",
                        "入账总计",
                        "支出总计",
                        "退款总计",
                        "余额",
                        "实际余额"
                    ])?;
                    sw.append_row(row![
                        *id as f64,
                        name.as_str(),
                        start,
                        incoming,
                        outgoing,
                        refund,
                        balance_c,
                        balance
                    ])?;
                    Ok(())
                })
                .unwrap();
                let mut sheet1 = wb.create_sheet("sheet2");
                wb.write_sheet(&mut sheet1, move |sheet_writer| {
                    let sw = sheet_writer;

                    sw.append_row(row!["入账明细"])?;
                    sw.append_row(row![
                        "日期",
                        "收款人账号",
                        "付款人账号",
                        "收款人名称",
                        "付款人名称",
                        "金额",
                        "用途",
                        "备注",
                        "附言"
                    ])?;
                    for row in incomings {
                        sw.append_row(row)?;
                    }
                    sw.append_row(row!["支出明细"])?;
                    sw.append_row(row!["账号", "账户", "日期", "金额"])?;
                    for row in outgoings {
                        sw.append_row(row)?;
                    }
                    Ok(())
                })
                .unwrap();

                let mut sheet2 = wb.create_sheet("sheet3");
                wb.write_sheet(&mut sheet2, move |sheet_writer| {
                    let sw = sheet_writer;
                    sw.append_row(row!["退款明细"])?;
                    sw.append_row(row!["账号", "账户", "日期", "金额"])?;
                    for row in refunds {
                        sw.append_row(row)?;
                    }
                    Ok(())
                })
                .unwrap();
                wb.close().expect("close excel error!");
                (id, name, start + incoming - outgoing + refund, balance)
            })
            .collect::<Vec<_>>();

        println!("{a:#?}");
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
