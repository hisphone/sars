use sub_account_reconcile_system::{
    balance::Balances,
    from_file::{FromExcel, FromTxt},
    incoming::Incomings,
    outgoing::Outgoings,
    refund::Refunds,
    start::Starts,
    sub_account::{SubAccount, SubInfos},
    // to_excel::IntoExcel,
};

fn main() {
    let sub_account = SubAccount::new(
        SubInfos::from_excel(r"F:\下载\虚拟子账户明细 .xls").unwrap(),
        Starts::from_excel(r"F:\下载\科目余额表20220214155436.xlsx").unwrap(),
        Incomings::from_txt("收入.txt").unwrap(),
        Outgoings::from_excel(r"F:\下载\财政云清算支出.xltx").unwrap(),
        Refunds::from_excel(r"F:\下载\财政云退款.xltx").unwrap(),
        Balances::from_txt("余额.txt").unwrap(),
    );
    println!("{sub_account:#?}");
}
