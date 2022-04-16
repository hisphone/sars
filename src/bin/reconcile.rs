use sub_account_reconcile_system::{
    balance::Balances, from_file::FromFormatedExcel, incoming::Incomings, outgoing::Outgoings,
    refund::Refunds, start::Starts, sub_account::SubAccount, sub_info::SubInfos,
};

fn main() {
    let mut path = std::env::current_exe().unwrap();
    path.set_file_name("formatt");

    let sub_account = SubAccount::new(
        SubInfos::from_formated_excel(path.join("子账户信息表.xlsx")).unwrap(),
        Starts::from_formated_excel(path.join("年初余额表.xlsx")).unwrap(),
        Incomings::from_formated_excel(path.join("入账信息表.xlsx")).unwrap(),
        Outgoings::from_formated_excel(path.join("支出信息表.xlsx")).unwrap(),
        Refunds::from_formated_excel(path.join("退款信息表.xlsx")).unwrap(),
        Balances::from_formated_excel(path.join("子账户余额表.xlsx")).unwrap(),
    );
    sub_account.reconcile();
}
