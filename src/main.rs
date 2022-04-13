use sub_account_reconcile_system::{balance::Balances, from_txt::FromTxt};

fn main() {
    let a = Balances::from_txt("王静 copy.txt").unwrap();
    println!("{a:#?}")
}
