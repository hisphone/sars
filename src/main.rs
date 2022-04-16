use std::{thread::sleep, time::Duration};

use sub_account_reconcile_system::{
    balance::Balances,
    from_file::{FromExcel, FromTxt},
    incoming::Incomings,
    outgoing::Outgoings,
    refund::Refunds,
    start::Starts,
    sub_account::SubAccount,
    sub_info::SubInfos,
    // to_excel::IntoExcel,
};

fn main() {
    sleep(Duration::from_secs(10));
    let sub_account = SubAccount::new(
        SubInfos::from_excel("/Users/mac/Desktop/1/虚拟子账户明细.xls").unwrap(),
        Starts::from_excel("/Users/mac/Desktop/1/科目余额表20220214155436.xlsx").unwrap(),
        Incomings::from_txt("/Users/mac/Desktop/1/收入.txt").unwrap(),
        Outgoings::from_excel("/Users/mac/Desktop/1/财政云清算支出.xltx").unwrap(),
        Refunds::from_excel("/Users/mac/Desktop/1/财政云退款.xltx").unwrap(),
        Balances::from_txt("/Users/mac/Desktop/1/余额.txt").unwrap(),
    );
    sub_account.reconcile();
}
