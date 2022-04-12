use sub_account_reconcile_system::{from_txt::FromTxt, incoming::Incomings};

fn main() {
    let a = Incomings::from_txt("王静.txt")
        .unwrap()
        .as_ref()
        .iter()
        .count();
    // let b = Incoming::from_str(&a);
    println!("{a:#?}")
}
