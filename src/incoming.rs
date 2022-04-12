use anyhow::Result;
use std::{num::ParseFloatError, str::FromStr};

use crate::from_txt::{FromTxt, TxtReader};

#[derive(Debug, Default)]
pub struct Incomings(Vec<Incoming>);

impl<P> TxtReader<P> for Incomings where P: AsRef<std::path::Path> {}

impl<P> FromTxt<P> for Incomings where P: AsRef<std::path::Path> {}

impl AsRef<Vec<Incoming>> for Incomings {
    fn as_ref(&self) -> &Vec<Incoming> {
        &self.0
    }
}

impl AsMut<Vec<Incoming>> for Incomings {
    fn as_mut(&mut self) -> &mut Vec<Incoming> {
        &mut self.0
    }
}

#[derive(thiserror::Error, Debug)]
pub enum IncomingFromStrErr {
    #[error("KnownLine: `{0}`")]
    KNOWN(String),
    #[error("ParseFloatErr: `{0}`")]
    ParseFloatError(ParseFloatError),
}

impl FromStr for Incomings {
    type Err = IncomingFromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut incomings = Incomings::default();
        for split in s.split("1\n\n") {
            if !split.is_empty() {
                incomings.as_mut().push(Incoming::from_str(split)?)
            }
        }
        Ok(incomings)
    }
}
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
    金额: f64,
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
impl FromStr for Incoming {
    type Err = IncomingFromStrErr;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut incoming = Incoming::default();
        let mut temp1 = String::new();
        let mut temp2 = String::new();
        let mut temp3:Vec<String> = Vec::new();
        for line in s.lines().rev() {
            incoming.set_fields(IncomingFilter::from_str(line)?, &mut (&mut temp1, &mut temp2, &mut temp3))?
        }
        Ok(incoming)
    }
}
impl Incoming {
    pub fn set_fields(
        &mut self,
        filter: IncomingFilter,
        temp:&mut (&mut String, &mut String, &mut Vec<String>)
    ) -> std::result::Result<(), IncomingFromStrErr> {
        match filter {
            IncomingFilter::UNUSEABLE => Ok(()),
            IncomingFilter::Fields(s) => {
                let mut fields = s.split_whitespace().map(String::from).collect::<Vec<_>>();
                if !temp.0.is_empty() || !temp.1.is_empty(){
                    fields[0] += temp.0;
                    fields[1] += temp.1;
                    (*temp).0.clear();
                    (*temp).1.clear();
                }
                for field in fields.iter() {
                    self.set_fields(IncomingFilter::from_str(field)?,&mut *temp)?
                }
                Ok(())
            },
            IncomingFilter::Extra1(s) => {*temp.1 = s.chars().filter(|c|!c.is_whitespace()).collect();Ok(())},
            IncomingFilter::Extra2(s) => {
                let mut split = s.split("\u{3000}").skip(1);
                *temp.0 = split.next().unwrap().to_string();
                *temp.1 = split.next().unwrap().to_string();
                Ok(())
            },
            IncomingFilter::客户号(setter) => {self.set_客户号(setter);Ok(())},
            IncomingFilter::日期(setter) => {self.set_日期(setter);Ok(())},
            IncomingFilter::收款人账号(setter) => {self.set_收款人账号(setter);Ok(())},
            IncomingFilter::付款人账号(setter) => {self.set_付款人账号(setter);Ok(())},
            IncomingFilter::收款人名称(setter) => {self.set_收款人名称(setter);Ok(())},
            IncomingFilter::付款人名称(setter) => {self.set_付款人名称(setter);Ok(())},
            IncomingFilter::收款人开户行(setter) => {self.set_收款人开户行(setter);Ok(())},
            IncomingFilter::付款人开户行(setter) => {self.set_付款人开户行(setter);Ok(())},
            IncomingFilter::金额(setter) => {self.set_金额(setter);Ok(())},
            IncomingFilter::金额大写(setter) => {self.set_金额大写(setter);Ok(())},
            IncomingFilter::报文种类(setter) => {self.set_报文种类(setter);Ok(())},
            IncomingFilter::业务类型(setter) => {self.set_业务类型(setter);Ok(())},
            IncomingFilter::收支申报号(setter) => {self.set_收支申报号(setter);Ok(())},
            IncomingFilter::业务标识号(setter) => {self.set_业务标识号(setter);Ok(())},
            IncomingFilter::业务编号(setter) => {self.set_业务编号(setter);Ok(())},
            IncomingFilter::发起行行号(setter) => {self.set_发起行行号(setter);Ok(())},
            IncomingFilter::接收行行号(setter) => {self.set_接收行行号(setter);Ok(())},
            IncomingFilter::发起行名称(setter) => {self.set_发起行名称(setter);Ok(())},
            IncomingFilter::接收行名称(setter) => {self.set_接收行名称(setter);Ok(())},
            IncomingFilter::入账账号(setter) => {self.set_入账账号(setter);Ok(())},
            IncomingFilter::入账户名(setter) => {self.set_入账户名(setter);Ok(())},
            IncomingFilter::用途(setter) => {self.set_用途(setter);Ok(())},
            IncomingFilter::附言(setter) => {self.set_附言(setter);Ok(())},
            IncomingFilter::交易机构(setter) => {self.set_交易机构(setter);Ok(())},
            IncomingFilter::交易渠道(setter) => {self.set_交易渠道(setter);Ok(())},
            IncomingFilter::交易流水号(setter) => {self.set_交易流水号(setter);Ok(())},
            IncomingFilter::经办(setter) => {self.set_经办(setter);Ok(())},
            IncomingFilter::回单编号(setter) => {self.set_回单编号(setter);Ok(())},
            IncomingFilter::回单验证码(setter) => {self.set_回单验证码(setter);Ok(())},
            IncomingFilter::打印时间(setter) => {self.set_打印时间(setter);Ok(())},
            IncomingFilter::打印次数(setter) => {self.set_打印次数(setter);Ok(())},
            IncomingFilter::业务种类(setter) => {self.set_业务种类(setter);Ok(())},
            IncomingFilter::凭证号码(setter) => {self.set_凭证号码(setter);Ok(())},
            IncomingFilter::备注(setter) => {self.set_备注(setter);Ok(())},
        }
    }
    pub fn set_客户号(&mut self,setter: String){
        self.客户号 = setter
    }
    pub fn set_日期(&mut self,setter: String){        
        self.日期 = setter
    }
    pub fn set_收款人账号(&mut self,setter: String){
        self.收款人账号 = setter
    }
    pub fn set_付款人账号(&mut self,setter: String){
        self.付款人账号 = setter
    }
    pub fn set_收款人名称(&mut self,setter: String){
        self.收款人名称 = setter
    }
    pub fn set_付款人名称(&mut self,setter: String){
        self.付款人名称 = setter
    }
    pub fn set_收款人开户行(&mut self,setter: String){
        self.收款人开户行 = setter
    }
    pub fn set_付款人开户行(&mut self,setter: String){
        self.付款人开户行 = setter
    }
    pub fn set_金额(&mut self,setter: f64){
        self.金额 = setter
    }
    pub fn set_金额大写(&mut self,setter: String){
        self.金额大写 = setter
    }
    pub fn set_用途(&mut self,setter: String){
        self.用途 = setter
    }
    pub fn set_备注(&mut self,setter: String){
        self.备注 = setter
    }
    pub fn set_附言(&mut self,setter: String){
        self.附言 = setter
    }
    pub fn set_报文种类(&mut self,setter: String){
        self.报文种类 = setter
    }
    pub fn set_业务类型(&mut self,setter: String){
        self.业务类型 = setter
    }
    pub fn set_业务种类(&mut self,setter: String){
        self.业务种类 = setter
    }
    pub fn set_凭证号码(&mut self,setter: String){
        self.凭证号码 = setter
    }
    pub fn set_收支申报号(&mut self,setter: String){
        self.收支申报号 = setter
    }
    pub fn set_业务标识号(&mut self,setter: String){
        self.业务标识号 = setter
    }
    pub fn set_业务编号(&mut self,setter: String){
        self.业务编号 = setter
    }
    pub fn set_发起行行号(&mut self,setter: String){
        self.发起行行号 = setter
    }
    pub fn set_接收行行号(&mut self,setter: String){
        self.接收行行号 = setter
    }
    pub fn set_发起行名称(&mut self,setter: String){
        self.发起行名称 = setter
    }
    pub fn set_接收行名称(&mut self,setter: String){
        self.接收行名称 = setter
    }
    pub fn set_入账账号(&mut self,setter: String){
        self.入账账号 = setter
    }
    pub fn set_入账户名(&mut self,setter: String){
        self.入账户名 = setter
    }
    pub fn set_交易机构(&mut self,setter: String){
        self.交易机构 = setter
    }
    pub fn set_交易渠道(&mut self,setter: String){
        self.交易渠道 = setter
    }
    pub fn set_交易流水号(&mut self,setter: String){
        self.交易流水号 = setter
    }
    pub fn set_经办(&mut self,setter: String){
        self.经办 = setter
    }
    pub fn set_回单编号(&mut self,setter: String){
        self.回单编号 = setter
    }
    pub fn set_回单验证码(&mut self,setter: String){
        self.回单验证码 = setter
    }
    pub fn set_打印时间(&mut self,setter: String){
        self.打印时间 = setter
    }
    pub fn set_打印次数(&mut self,setter: String){
        self.打印次数 = setter
    }
}
pub enum IncomingFilter {
    UNUSEABLE,
    Fields(String),
    Extra1(String),
    Extra2(String),
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
impl FromStr for IncomingFilter {
    type Err = IncomingFromStrErr;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            ""
            | " "
            | "1"
            | "次"
            | " --------------------------------------------------------------------------------------------------------"
            | "                                             国内支付业务收款回单"
            | " 本机构吸收的本外币存款依照《存款保险条例》受到保护。"
            | "             \u{3000}                                                    \u{3000}                                    \u{3000}"
            | "           \u{3000}                                                    \u{3000}                                      \u{3000}"
            | "           \u{3000}                                                    \u{3000}                                        "
            | "                                                               \u{3000}                                        \u{3000}"
            | "  \u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}自助打印，请避免重复\u{3000}"
            | "  \u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}自助打印，请避免重复" => {
                Ok(IncomingFilter::UNUSEABLE)
            }
            s if s.starts_with("             \u{3000}")|s.starts_with("           \u{3000}") => Ok(
                IncomingFilter::Extra2(s.to_string()),
            ),
            s if s.starts_with("                                                               \u{3000}")|s.starts_with("           \u{3000}}") => Ok(
                IncomingFilter::Extra1(s.to_string()),
            ),
            s if s.starts_with("客户号：") => Ok(
                IncomingFilter::客户号(s.strip_prefix("客户号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("日期：") => Ok(
                IncomingFilter::日期(s.strip_prefix("日期：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("收款人账号：") => Ok(
                IncomingFilter::收款人账号(s.strip_prefix("收款人账号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("付款人账号：") => Ok(
                IncomingFilter::付款人账号(s.strip_prefix("付款人账号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("收款人名称：") => Ok(
                IncomingFilter::收款人名称(s.strip_prefix("收款人名称：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("付款人名称：") => Ok(
                IncomingFilter::付款人名称(s.strip_prefix("付款人名称：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("收款人开户行：") => Ok(
                IncomingFilter::收款人开户行(s.strip_prefix("收款人开户行：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("付款人开户行：") => Ok(
                IncomingFilter::付款人开户行(s.strip_prefix("付款人开户行：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with(" 金额：") => Ok(
                IncomingFilter::金额(s.chars().filter(|c| *c == '.' || c.is_ascii_digit()).collect::<String>().parse::<f64>().map_err(|e|IncomingFromStrErr::ParseFloatError(e))?),
            ),
            s if s.starts_with("       人民币") => Ok(
                IncomingFilter::金额大写(s.replace("       人民币", "")),
            ),
            s if s.starts_with(" 报文种类：") => Ok(
                IncomingFilter::报文种类(s.strip_prefix(" 报文种类：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("业务种类：") => Ok(
                IncomingFilter::业务种类(s.strip_prefix("业务种类：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("业务编号：") => Ok(
                IncomingFilter::业务编号(s.strip_prefix("业务编号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("凭证号码：") => Ok(
                IncomingFilter::凭证号码(s.strip_prefix("凭证号码：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("业务类型：") => Ok(
                IncomingFilter::业务类型(s.strip_prefix("业务类型：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("收支申报号：") => Ok(
                IncomingFilter::收支申报号(s.strip_prefix("收支申报号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("业务标识号：") => Ok(
                IncomingFilter::业务标识号(s.strip_prefix("业务标识号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("发起行行号：") => Ok(
                IncomingFilter::发起行行号(s.strip_prefix("发起行行号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("接收行行号：") => Ok(
                IncomingFilter::接收行行号(s.strip_prefix("接收行行号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("发起行名称：") => Ok(
                IncomingFilter::发起行名称(s.strip_prefix("发起行名称：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("接收行名称：") => Ok(
                IncomingFilter::接收行名称(s.strip_prefix("接收行名称：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("入账账号：") => Ok(
                IncomingFilter::入账账号(s.strip_prefix("入账账号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("入账户名：") => Ok(
                IncomingFilter::入账户名(s.strip_prefix("入账户名：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with(" 用途：") => Ok(
                IncomingFilter::用途(s.strip_prefix(" 用途：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with(" 备注：") => Ok(
                IncomingFilter::备注(s.strip_prefix(" 备注：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with(" 附言：") => Ok(
                IncomingFilter::附言(s.strip_prefix(" 附言：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("交易机构：") => Ok(
                IncomingFilter::交易机构(s.strip_prefix("交易机构：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("交易渠道：") => Ok(
                IncomingFilter::交易渠道(s.strip_prefix("交易渠道：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("交易流水号：") => Ok(
                IncomingFilter::交易流水号(s.strip_prefix("交易流水号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("经办：") => Ok(
                IncomingFilter::经办(s.strip_prefix("经办：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("回单编号：") => Ok(
                IncomingFilter::回单编号(s.strip_prefix("回单编号：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("回单验证码：") => Ok(
                IncomingFilter::回单验证码(s.strip_prefix("回单验证码：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("打印时间：") => Ok(
                IncomingFilter::打印时间(s.strip_prefix("打印时间：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with("打印次数：") => Ok(
                IncomingFilter::打印次数(s.strip_prefix("打印次数：").unwrap().trim_end().to_string()),
            ),
            s if s.starts_with(" 业务类型：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            s if s.starts_with(" 业务标识号：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            s if s.starts_with(" 发起行行号：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            s if s.starts_with(" 发起行名称：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            s if s.starts_with(" 入账账号：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            s if s.starts_with("               客户号：") |s.starts_with("                 客户号：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            s if s.starts_with(" 收款人账号：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            s if s.starts_with(" 收款人名称：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            s if s.starts_with(" 收款人开户行：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),            
            s if s.starts_with(" 业务种类：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            
            s if s.starts_with(" 交易机构：") => Ok(
                IncomingFilter::Fields(s.to_string()),
            ),
            s if s.starts_with(" 回单编号：") => {
                Ok(IncomingFilter::Fields(s.to_string()))
            }            
            s => Err(IncomingFromStrErr::KNOWN(s.to_string())),
        }
    }
}
