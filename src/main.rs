use sub_account_reconcile_system::sub_account::SubInfos;

fn main() {
    let a = SubInfos::from_excel(r"F:\下载\虚拟子账户明细 .xls").unwrap();

    println!("{:?}", a.as_ref().iter().count());
}
