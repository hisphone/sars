use anyhow::Ok;
use sub_account_reconcile_system::{balance::Balances, from_file::FromTxt, to_excel::ToExcel};

fn main() -> anyhow::Result<()> {
    let path = std::env::current_exe()?.parent().unwrap().to_path_buf();

    println!("开始读取{:?}", path.join("余额.txt"));
    let balance = Balances::from_txt(&path.join("余额.txt"))?;

    println!("开始写入{:?}", path.join("formatt/子账户余额表.xlsx"));
    balance.to_excel(&path.join("formatt/子账户余额表.xlsx").to_str().unwrap())?;
    Ok(())
}
