use std::{num::ParseFloatError, str::FromStr};

use crate::from_txt::TxtReader;

pub type Incomings = Vec<Incoming>;

#[derive(Debug, Default)]
pub struct Incoming {
    客户号: String,
    日期: String,
    收款人账号: String,
    付款人账号: String,
    收款人名称: String,
    付款人名称: String,
    收款人开户行: String,
    付款人开户行: String,
    金额: String,
    金额大写: String,
    用途: String,
    备注: String,
    附言: String,
    报文种类: String,
    业务类型: String,
    业务种类: String,
    凭证号码: String,
    收支申报号: String,
    业务标识号: String,
    业务编号: String,
    发起行行号: String,
    接收行行号: String,
    发起行名称: String,
    接收行名称: String,
    入账账号: String,
    入账户名: String,
    交易机构: String,
    交易渠道: String,
    交易流水号: String,
    经办: String,
    回单编号: String,
    回单验证码: String,
    打印时间: String,
    打印次数: String,
}
// impl<P: AsRef<std::path::Path>> FromTxt<P> for Incoming {
//     type Filter = Filter;

//     fn handler(filter: Self::Filter, target: &mut Self) -> anyhow::Result<()> {
//         match filter {
//             Filter::UNUSEABLE => Ok(()),
//             Filter::客户号(_) => Ok(()),
//             Filter::日期(_) => Ok(()),
//             Filter::收款人账号(_) => Ok(()),
//             Filter::付款人账号(_) => Ok(()),
//             Filter::收款人名称(_) => Ok(()),
//             Filter::付款人名称(_) => Ok(()),
//             Filter::收款人开户行(_) => Ok(()),
//             Filter::付款人开户行(_) => Ok(()),
//             Filter::金额(_) => Ok(()),
//             Filter::金额大写(_) => Ok(()),
//             Filter::报文种类(_) => Ok(()),
//             Filter::业务类型(_) => Ok(()),
//             Filter::收支申报号(_) => Ok(()),
//             Filter::业务标识号(_) => Ok(()),
//             Filter::业务编号(_) => Ok(()),
//             Filter::发起行行号(_) => Ok(()),
//             Filter::接收行行号(_) => Ok(()),
//             Filter::发起行名称(_) => Ok(()),
//             Filter::接收行名称(_) => Ok(()),
//             Filter::入账账号(_) => Ok(()),
//             Filter::入账户名(_) => Ok(()),
//             Filter::用途(_) => Ok(()),
//             Filter::附言(_) => Ok(()),
//             Filter::交易机构(_) => Ok(()),
//             Filter::交易渠道(_) => Ok(()),
//             Filter::交易流水号(_) => Ok(()),
//             Filter::经办(_) => Ok(()),
//             Filter::回单编号(_) => Ok(()),
//             Filter::回单验证码(_) => Ok(()),
//             Filter::打印时间(_) => Ok(()),
//             Filter::打印次数(_) => Ok(()),
//             Filter::业务种类(_) => Ok(()),
//             Filter::凭证号码(_) => Ok(()),
//             Filter::备注(_) => Ok(()),
//             Filter::回单编号_回单验证码_打印时间_打印次数(_) => Ok(()),
//             Filter::交易机构_交易渠道_交易流水号_经办(_) => Ok(()),
//             Filter::业务种类_业务编号_凭证号码(_) => Ok(()),
//             Filter::收款人开户行_付款人开户行(_) => Ok(()),
//             Filter::收款人名称_付款人名称(_) => Ok(()),
//             Filter::收款人账号_付款人账号(_) => Ok(()),
//             Filter::客户号_日期(_) => Ok(()),
//             Filter::入账账号_入账账户(_) => Ok(()),
//             Filter::发起行名称_接受行名称(_) => Ok(()),
//             Filter::发起行行号_接受行行号(_) => Ok(()),
//             Filter::业务标识号_业务编号(_) => Ok(()),
//             Filter::业务类型_收支申报号(_) => Ok(()),
//         }
//     }
// }
impl<P: AsRef<std::path::Path>> TxtReader<P> for Incoming {}
pub enum Filter {
    UNUSEABLE,
    业务类型_收支申报号(String),
    业务标识号_业务编号(String),
    发起行行号_接受行行号(String),
    发起行名称_接受行名称(String),
    入账账号_入账账户(String),
    客户号_日期(String),
    收款人账号_付款人账号(String),
    收款人名称_付款人名称(String),
    收款人开户行_付款人开户行(String),
    回单编号_回单验证码_打印时间_打印次数(String),
    交易机构_交易渠道_交易流水号_经办(String),
    业务种类_业务编号_凭证号码(String),
    客户号(String),
    日期(String),
    收款人账号(String),
    付款人账号(String),
    收款人名称(String),
    付款人名称(String),
    收款人开户行(String),
    付款人开户行(String),
    金额(f64),
    金额大写(String),
    报文种类(String),
    业务类型(String),
    收支申报号(String),
    业务标识号(String),
    业务编号(String),
    发起行行号(String),
    接收行行号(String),
    发起行名称(String),
    接收行名称(String),
    入账账号(String),
    入账户名(String),
    用途(String),
    附言(String),
    交易机构(String),
    交易渠道(String),
    交易流水号(String),
    经办(String),
    回单编号(String),
    回单验证码(String),
    打印时间(String),
    打印次数(String),
    业务种类(String),
    凭证号码(String),
    备注(String),
}

// impl FromStr for Filter {
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             ""
//             | "1"
//             | " --------------------------------------------------------------------------------------------------------"
//             | "                                             国内支付业务收款回单"
//             | "             \u{3000}                                                    \u{3000}                                    \u{3000}"
//             | "           \u{3000}                                                    \u{3000}                                      \u{3000}"
//             | "           \u{3000}                                                    \u{3000}                                        "
//             | "                                                               \u{3000}                                        \u{3000}"
//             | "  \u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}自助打印，请避免重复\u{3000}"
//             | "  \u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}自助打印，请避免重复" => {
//                 Ok(Filter::UNUSEABLE)
//             }
//             s if s.starts_with(" 报文种类：") => Ok(
//                 Filter::报文种类(s.strip_prefix(" 报文种类：").unwrap().trim_end().to_string()),
//             ),
//             s if s.starts_with(" 业务类型：") => Ok(
//                 Filter::业务类型_收支申报号(s.to_string()),
//             ),
//             s if s.starts_with(" 业务标识号：") => Ok(
//                 Filter::业务标识号_业务编号(s.to_string()),
//             ),
//             s if s.starts_with(" 发起行行号：") => Ok(
//                 Filter::发起行行号_接受行行号(s.to_string()),
//             ),
//             s if s.starts_with(" 发起行名称：") => Ok(
//                 Filter::发起行名称_接受行名称(s.to_string()),
//             ),
//             s if s.starts_with(" 入账账号：") => Ok(
//                 Filter::入账账号_入账账户(s.to_string()),
//             ),
//             s if s.starts_with("               客户号：") => Ok(
//                 Filter::客户号_日期(s.to_string()),
//             ),
//             s if s.starts_with(" 收款人账号：") => Ok(
//                 Filter::收款人账号_付款人账号(s.to_string()),
//             ),
//             s if s.starts_with(" 收款人名称：") => Ok(
//                 Filter::收款人名称_付款人名称(s.to_string()),
//             ),
//             s if s.starts_with(" 收款人开户行：") => Ok(
//                 Filter::收款人开户行_付款人开户行(s.to_string()),
//             ),
//             s if s.starts_with(" 金额：") => Ok(
//                 Filter::金额(s.chars().filter(|c| *c == '.' || c.is_ascii_digit()).collect::<String>().parse::<f64>().map_err(|e|FilterErr::ParseFloatError(e))?),
//             ),
//             s if s.starts_with("       人民币") => Ok(
//                 Filter::金额大写(s.replace("       人民币", "")),
//             ),
//             s if s.starts_with(" 业务种类：") => Ok(
//                 Filter::业务种类_业务编号_凭证号码(s.to_string()),
//             ),
//             s if s.starts_with(" 用途：") => Ok(
//                 Filter::用途(s.strip_prefix(" 用途：").unwrap().trim_end().to_string()),
//             ),
//             s if s.starts_with(" 备注：") => Ok(
//                 Filter::备注(s.strip_prefix(" 备注：").unwrap().trim_end().to_string()),
//             ),
//             s if s.starts_with(" 附言：") => Ok(
//                 Filter::附言(s.strip_prefix(" 附言：").unwrap().trim_end().to_string()),
//             ),
//             s if s.starts_with(" 交易机构：") => Ok(
//                 Filter::交易机构_交易渠道_交易流水号_经办(s.to_string()),
//             ),
//             s if s.starts_with(" 回单编号：") => {
//                 Ok(Filter::回单编号_回单验证码_打印时间_打印次数(s.to_string()))
//             }
//             s => Err(FilterErr::KNOWN(s.to_string())),
//         }
//     }

//     type Err = FilterErr;
// }
#[derive(thiserror::Error, Debug)]
pub enum TxtFilterErr {
    #[error("KnownLine: `{0}`")]
    KNOWN(String),
    #[error("ParseFloatErr: `{0}`")]
    ParseFloatError(ParseFloatError),
}
pub enum TxtFilter<'a> {
    Field(&'a str),
    Fields(&'a str),
    Extra(&'a str),
    UnUsable,
}
impl<'a> FromStr for TxtFilter<'a> {
    type Err = TxtFilterErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
