use sub_account_reconcile_system::{from_txt::FromTxt, incoming::Incomings, to_excel::IntoExcel};

fn main() {
    let a = Incomings::from_txt("王静.txt").unwrap();
    // let b = a
    //     .into_iter()
    //     .filter(|i| i.get_收款人账号() == "102831264647")
    //     .collect::<Vec<_>>();

    a.into_excel("xxx.xlsx")
}
