use sub_account_reconcile_system::{from_txt::FromTxt, incoming::Incomings};

fn main() {
    let a = Incomings::from_txt("王静.txt").unwrap();
    let b = a
        .as_ref()
        .iter()
        .filter(|i| i.get_收款人账号() == "102831264647")
        .collect::<Vec<_>>();

    println!("{a:#?}")
}
