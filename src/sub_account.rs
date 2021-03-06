use simple_excel_writer::{row, Column, Row, Workbook};

use crate::{
    balance::Balances, incoming::Incomings, outgoing::Outgoings, refund::Refunds, start::Starts,
    sub_info::SubInfos, to_excel::ToRow,
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
                let id = si.get_id();
                let name = si.get_name();
                let va = si.get_va();
                let start = match self
                    .get_start()
                    .as_ref()
                    .iter()
                    .find(|start| start.get_id() == id)
                {
                    Some(s) => s.get_start(),
                    None => 0.0,
                };
                let incoming_iter = self.get_incoming().iter().filter(|i| i.get_id() == id);
                let incoming = incoming_iter.clone().map(|i| i.get_ammount()).sum::<f64>();
                let incomings = incoming_iter.map(|i| i.to_row()).collect::<Vec<_>>();
                let outgoing_iter = self
                    .get_outgoing()
                    .as_ref()
                    .iter()
                    .filter(|o| o.get_id() == id);
                let outgoing = outgoing_iter.clone().map(|o| o.get_outgoing()).sum::<f64>();
                let outgoings = outgoing_iter.map(|o| o.to_row()).collect::<Vec<_>>();
                let refund_iter = self
                    .get_refund()
                    .as_ref()
                    .iter()
                    .filter(|r| r.get_id() == id);
                let refund = refund_iter.clone().map(|r| r.get_refund()).sum::<f64>();
                let refunds = refund_iter.map(|r| r.to_row()).collect::<Vec<_>>();
                let balance = match self
                    .get_balance()
                    .iter()
                    .find(|balance| balance.get_id() == id)
                {
                    Some(b) => b.get_balance(),
                    None => 0.0,
                };
                let balance_c = start + incoming - outgoing - refund;
                let mut wb = Workbook::create(
                    std::env::current_exe()
                        .unwrap()
                        .with_file_name(name.clone() + ".xlsx")
                        .to_str()
                        .unwrap(),
                );
                let mut sheet = wb.create_sheet("??????");
                sheet.add_column(Column { width: 20.0 });
                sheet.add_column(Column { width: 40.0 });
                sheet.add_column(Column { width: 15.0 });
                sheet.add_column(Column { width: 15.0 });
                sheet.add_column(Column { width: 15.0 });
                sheet.add_column(Column { width: 15.0 });
                sheet.add_column(Column { width: 15.0 });
                sheet.add_column(Column { width: 15.0 });
                wb.write_sheet(&mut sheet, move |sheet_writer| {
                    let sw = sheet_writer;
                    sw.append_row(row![
                        "??????",
                        "??????",
                        "????????????",
                        "????????????",
                        "????????????",
                        "????????????",
                        "??????"
                    ])?;
                    sw.append_row(row![
                        id.to_string(),
                        name.as_str(),
                        start,
                        incoming,
                        outgoing,
                        refund,
                        balance_c
                    ])?;
                    Ok(())
                })
                .unwrap();

                let mut sheet1 = wb.create_sheet("????????????");
                sheet1.add_column(Column { width: 15.0 });
                sheet1.add_column(Column { width: 19.0 });
                sheet1.add_column(Column { width: 35.0 });
                sheet1.add_column(Column { width: 19.0 });
                sheet1.add_column(Column { width: 35.0 });
                sheet1.add_column(Column { width: 15.0 });
                sheet1.add_column(Column { width: 25.0 });
                sheet1.add_column(Column { width: 25.0 });
                sheet1.add_column(Column { width: 25.0 });
                wb.write_sheet(&mut sheet1, move |sheet_writer| {
                    let sw = sheet_writer;

                    sw.append_row(row![
                        "??????",
                        "???????????????",
                        "???????????????",
                        "???????????????",
                        "???????????????",
                        "??????",
                        "??????",
                        "??????",
                        "??????"
                    ])?;
                    for row in incomings {
                        sw.append_row(row)?;
                    }
                    Ok(())
                })
                .unwrap();

                let mut sheet2 = wb.create_sheet("????????????");
                sheet2.add_column(Column { width: 20.0 });
                sheet2.add_column(Column { width: 20.0 });
                sheet2.add_column(Column { width: 40.0 });
                sheet2.add_column(Column { width: 20.0 });
                wb.write_sheet(&mut sheet2, move |sheet_writer| {
                    let sw = sheet_writer;
                    sw.append_row(row!["??????", "??????", "??????", "??????"])?;
                    for row in outgoings {
                        sw.append_row(row)?;
                    }
                    Ok(())
                })
                .unwrap();

                let mut sheet3 = wb.create_sheet("????????????");
                sheet3.add_column(Column { width: 20.0 });
                sheet3.add_column(Column { width: 20.0 });
                sheet3.add_column(Column { width: 40.0 });
                sheet3.add_column(Column { width: 20.0 });
                wb.write_sheet(&mut sheet3, move |sheet_writer| {
                    let sw = sheet_writer;
                    sw.append_row(row!["??????", "??????", "??????", "??????"])?;
                    for row in refunds {
                        sw.append_row(row)?;
                    }
                    Ok(())
                })
                .unwrap();
                wb.close().expect("close excel error!");
                (
                    id,
                    name,
                    start,
                    incoming,
                    outgoing,
                    refund,
                    start + incoming - outgoing - refund,
                    balance,
                    va,
                )
            })
            .collect::<Vec<_>>();
        let mut wb = Workbook::create(
            std::env::current_exe()
                .unwrap()
                .with_file_name("??????.xlsx")
                .to_str()
                .unwrap(),
        );
        let mut sheet = wb.create_sheet("?????????????????????");
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 40.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 20.0 });
        wb.write_sheet(&mut sheet, move |sheet_writer| {
            let sw = sheet_writer;

            sw.append_row(row![
                "????????????",
                "????????????",
                "????????????",
                "??????",
                "??????",
                "??????",
                "??????",
                "????????????"
            ])?;
            for row in a {
                sw.append_row(row![
                    row.0.to_string(),
                    row.1.as_ref(),
                    row.2,
                    row.3,
                    row.4,
                    row.5,
                    row.6,
                    row.7,
                    row.8.as_ref()
                ])?;
            }
            Ok(())
        })
        .unwrap();
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
