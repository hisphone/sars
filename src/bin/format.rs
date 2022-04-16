use sub_account_reconcile_system::{
    balance::Balances,
    from_file::{FromExcel, FromTxt},
    incoming::Incomings,
    outgoing::Outgoings,
    refund::Refunds,
    start::Starts,
    sub_info::SubInfos,
    to_excel::ToExcel,
};

fn main() -> anyhow::Result<()> {
    let mut path = std::env::current_exe()?;
    path.pop();

    println!("开始读取{:?}", path.join("子账户信息.xls"));
    let sub_info = SubInfos::from_excel(&path.join("子账户信息.xls"))?;

    println!("开始写入{:?}", path.join("formatt/子账户信息表.xlsx"));
    sub_info.to_excel(&path.join("formatt/子账户信息表.xlsx").to_str().unwrap())?;

    println!("开始读取{:?}", path.join("年初余额.xlsx"));
    let start = Starts::from_excel(&path.join("年初余额.xlsx"))?;

    println!("开始写入{:?}", path.join("formatt/年初余额表.xlsx"));
    start.to_excel(&path.join("formatt/年初余额表.xlsx").to_str().unwrap())?;

    println!("开始读取{:?}", path.join("收入.txt"));
    let incoming = Incomings::from_txt(path.join("收入.txt"))?;

    println!("开始写入{:?}", path.join("formatt/入账信息表.xlsx"));
    incoming.to_excel(&path.join("formatt/入账信息表.xlsx").to_str().unwrap())?;

    println!("开始读取{:?}", path.join("支出.xls"));
    let outgoing = Outgoings::from_excel(&path.join("支出.xls"))?;

    println!("开始写入{:?}", path.join("formatt/支出信息表.xlsx"));
    outgoing.to_excel(&path.join("formatt/支出信息表.xlsx").to_str().unwrap())?;

    println!("开始读取{:?}", path.join("退款.xls"));
    let refunds = Refunds::from_excel(&path.join("退款.xls"))?;

    println!("开始写入{:?}", path.join("formatt/退款信息表.xlsx"));
    refunds.to_excel(&path.join("formatt/退款信息表.xlsx").to_str().unwrap())?;

    println!("开始读取{:?}", path.join("余额.txt"));
    let balance = Balances::from_txt(&path.join("余额.txt"))?;

    println!("开始写入{:?}", path.join("formatt/子账户余额表.xlsx"));
    balance.to_excel(&path.join("formatt/子账户余额表.xlsx").to_str().unwrap())?;

    Ok(())
}
