use sub_account_reconcile_system::{
    balance::Balances,
    from_file::{FromExcel, FromTxt},
    incoming::Incomings,
    outgoing::Outgoings,
    refund::Refunds,
    start::Starts,
    sub_account::SubInfos,
    to_excel::IntoExcel,
};

fn main() {
    let sub_infos = SubInfos::from_excel("虚拟子账户明细 .xls").unwrap();
    // let starts = Starts::from_excel(
    //     r"E:\project\sub_account_reconcile_system\科目余额表20220214155436.xlsx",
    // )
    // .unwrap();
    // let outgoings = Outgoings::from_excel(r"F:\下载\财政云清算支出.xltx").unwrap();
    let refunds = Refunds::from_excel(r"F:\下载\财政云退款.xltx").unwrap();
    println!("{refunds:#?}")
    // let incomings = Incomings::from_txt("4.txt").unwrap();
    // let balances = Balances::from_txt("余额.txt").unwrap();
    // incomings.into_excel("收入4.xlsx");
    // balances.into_excel("余额.xlsx");
    // let ids = starts
    //     .as_ref()
    //     .iter()
    //     .map(|s| s.get_id())
    //     .collect::<Vec<_>>();
    // sub_infos.as_ref().iter().map(|si| si.id).for_each(|id| {
    //     if !ids.contains(&id) {
    //         println!("{id}")
    //     }
    // })
}
