use anyhow::Result;
use simple_excel_writer::{row, Row};
use std::{
    convert::Infallible,
    io::Read,
    slice::{Iter, IterMut},
    str::FromStr,
    vec::IntoIter,
};

use crate::{
    from_file::{FromTxt, TxtReader},
    to_excel::{Header, IntoExcel, Title, ToRow},
};

const UNUSEABLE:[&str;12] = ["", " ", "1", " --------------------------------------------------------------------------------------------------------", "                                             国内支付业务收款回单"," 本机构吸收的本外币存款依照《存款保险条例》受到保护。", "             \u{3000}                                                    \u{3000}                                    \u{3000}", "           \u{3000}                                                    \u{3000}                                      \u{3000}", "           \u{3000}                                                    \u{3000}                                        ", "                                                               \u{3000}                                        \u{3000}", "  \u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}自助打印，请避免重复", "  \u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}自助打印，请避免重复\u{3000}"];
pub const FIELDS: [&str; 34] = [
    "客户号：",
    "日期：",
    "收款人账号：",
    "付款人账号：",
    "收款人名称：",
    "付款人名称：",
    "收款人开户行：",
    "付款人开户行：",
    "金额：",
    "金额大写：",
    "报文种类：",
    "业务类型：",
    "收支申报号：",
    "业务标识号：",
    "业务编号：",
    "发起行行号：",
    "接收行行号：",
    "发起行名称：",
    "接收行名称：",
    "入账账号：",
    "入账户名：",
    "用途：",
    "附言：",
    "交易机构：",
    "交易渠道：",
    "交易流水号：",
    "经办：",
    "回单编号：",
    "回单验证码：",
    "打印时间：",
    "打印次数：",
    "业务种类：",
    "凭证号码：",
    "备注：",
];

#[derive(Debug, Default)]
pub struct Incomings(Vec<Incoming>);

impl Incomings {
    pub fn iter(&self) -> Iter<'_, Incoming> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, Incoming> {
        self.into_iter()
    }
}

impl<'a> Header for &'a Incomings {
    fn header(&self) -> Row {
        row![
            "日期",
            "收款人账号",
            "付款人账号",
            "收款人名称",
            "付款人名称",
            "金额",
            "用途",
            "备注",
            "附言"
        ]
    }
}
impl<'a> Title for &'a Incomings {
    fn title(&self) -> &'static str {
        "收入"
    }
}
impl<'a> IntoExcel for &'a Incomings {}
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

impl<'a> IntoIterator for &'a Incomings {
    type Item = &'a Incoming;

    type IntoIter = Iter<'a, Incoming>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}
impl<'a> IntoIterator for &'a mut Incomings {
    type Item = &'a mut Incoming;

    type IntoIter = IterMut<'a, Incoming>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().iter_mut()
    }
}
impl FromStr for Incomings {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut incomings =
            vec![Incoming::default(); s.matches("国内支付业务收款回单").count()];
        let mut index = 0;
        let mut temp1 = String::new();
        let mut temp2 = String::new();
        let mut temp3 = Vec::new();
        let lines = s
            .lines()
            .rev()
            .filter_map(|line| match line {
                line if UNUSEABLE.iter().any(|&u| u == line) => None,
                line if line.starts_with("             \u{3000}")
                    | line.starts_with("           \u{3000}") =>
                {
                    let mut split = line.split("\u{3000}").skip(1);
                    temp1 = split.next().unwrap().to_string();
                    temp2 = split.next().unwrap().to_string();
                    None
                }
                line if line.starts_with(
                    "                                                               \u{3000}",
                ) =>
                {
                    temp2 = line.chars().filter(|c| !c.is_whitespace()).collect();
                    None
                }
                line if line.starts_with("       人民币") => {
                    Some(vec![line.replace("       人民币", "金额大写：")])
                }
                _ => {
                    let mut line = line
                        .split_whitespace()
                        .rev()
                        .filter_map(|s| {
                            if !FIELDS.iter().any(|f| s.starts_with(f)) {
                                temp3.push(s);
                                None
                            } else if temp3.is_empty() {
                                Some(s.to_string())
                            } else {
                                let s = s.to_string() + " " + &temp3.join(" ");
                                temp3.clear();
                                Some(s)
                            }
                        })
                        .collect::<Vec<_>>();
                    if !temp1.is_empty() && temp2.is_empty() {
                        line[0] += &temp1;
                        line[1] += &temp2;
                        temp1.clear();
                        temp2.clear();
                    }
                    Some(line)
                }
            })
            .flatten();
        for line in lines {
            match line {
                line if line.starts_with("客户号") => {
                    // incomings[index].set_客户号(line.split_once("：").unwrap().1.to_string());
                    index += 1;
                }
                line if line.starts_with("日期") => {
                    incomings[index].set_date(line.split_once("：").unwrap().1.to_string());
                }
                line if line.starts_with("收款人账号") => {
                    let account = line.split_once("：").unwrap().1;
                    let account = account
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>();
                    let account = if account != "102831264647" {
                        account.strip_prefix("102831264647").unwrap().to_string()
                    } else {
                        "0".to_string()
                    };
                    incomings[index].set_id(account.parse().unwrap())
                }
                line if line.starts_with("付款人账号") => {
                    incomings[index].set_from_id(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("收款人名称") => incomings[index]
                    .set_收款人名称(line.split_once("：").unwrap().1.to_string()),
                line if line.starts_with("付款人名称") => {
                    incomings[index].set_from_name(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("收款人开户行") => {
                    // incomings[index].set_收款人开户行(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("付款人开户行") => {
                    // incomings[index].set_付款人开户行(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("金额：") => incomings[index].set_ammount(
                    line.chars()
                        .filter(|c| c.is_ascii_digit() || *c == '.')
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                ),
                line if line.starts_with("金额大写") => {
                    // incomings[index].set_金额大写(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("用途") => {
                    incomings[index].set_usage(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("备注") => {
                    incomings[index].set_comment(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("附言") => {
                    incomings[index].set_postscript(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("报文种类") => {
                    // incomings[index].set_报文种类(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("业务类型") => {
                    // incomings[index].set_业务类型(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("业务种类") => {
                    // incomings[index].set_业务种类(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("凭证号码") => {
                    // incomings[index].set_凭证号码(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("收支申报号") => {
                    // incomings[index].set_收支申报号(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("业务标识号") => {
                    // incomings[index].set_业务标识号(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("业务编号") => {
                    // incomings[index].set_业务编号(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("发起行行号") => {
                    // incomings[index].set_发起行行号(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("接收行行号") => {
                    // incomings[index].set_接收行行号(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("发起行名称") => {
                    // incomings[index].set_发起行名称(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("接收行名称") => {
                    // incomings[index].set_接收行名称(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("入账账号") => {
                    // incomings[index].set_入账账号(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("入账户名") => {
                    // incomings[index].set_入账户名(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("交易机构") => {
                    // incomings[index].set_交易机构(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("交易渠道") => {
                    // incomings[index].set_交易渠道(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("交易流水号") => {
                    // incomings[index].set_交易流水号(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("经办") => {
                    // incomings[index].set_经办(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("回单编号") => {
                    // incomings[index].set_回单编号(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("回单验证码") => {
                    // incomings[index].set_回单验证码(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("打印时间") => {
                    // incomings[index].set_打印时间(line.split_once("：").unwrap().1.to_string())
                }
                line if line.starts_with("打印次数") => {
                    // incomings[index].set_打印次数(line.split_once("：").unwrap().1.to_string())
                }
                _ => {
                    println!("{line:?}");
                    panic!("KnownLine")
                }
            }
        }
        Ok(Self(incomings))
    }
}
#[derive(Debug, Default, Clone)]
pub struct Incoming {
    // 客户号: String,
    id: u32,
    date: String,
    from_id: String,
    name: String,
    from_name: String,
    // 收款人开户行: String,
    // 付款人开户行: String,
    ammount: f64,
    // 金额大写: String,
    usage: String,
    comment: String,
    postscript: String,
    // 报文种类: String,
    // 业务类型: String,
    // 业务种类: String,
    // 凭证号码: String,
    // 收支申报号: String,
    // 业务标识号: String,
    // 业务编号: String,
    // 发起行行号: String,
    // 接收行行号: String,
    // 发起行名称: String,
    // 接收行名称: String,
    // 入账账号: String,
    // 入账户名: String,
    // 交易机构: String,
    // 交易渠道: String,
    // 交易流水号: String,
    // 经办: String,
    // 回单编号: String,
    // 回单验证码: String,
    // 打印时间: String,
    // 打印次数: String,
}

impl Incoming {
    // pub fn set_客户号(&mut self, setter: String) {
    //     self.客户号 = setter
    // }
    pub fn set_id(&mut self, setter: u32) {
        self.id = setter
    }
    pub fn set_date(&mut self, setter: String) {
        self.date = setter
    }
    pub fn set_from_id(&mut self, setter: String) {
        self.from_id = setter
    }
    pub fn set_收款人名称(&mut self, setter: String) {
        self.name = setter
    }
    pub fn set_from_name(&mut self, setter: String) {
        self.from_name = setter
    }
    // pub fn set_收款人开户行(&mut self, setter: String) {
    //     self.收款人开户行 = setter
    // }
    // pub fn set_付款人开户行(&mut self, setter: String) {
    //     self.付款人开户行 = setter
    // }
    pub fn set_ammount(&mut self, setter: f64) {
        self.ammount = setter
    }
    // pub fn set_金额大写(&mut self, setter: String) {
    //     self.金额大写 = setter
    // }
    pub fn set_usage(&mut self, setter: String) {
        self.usage = setter
    }
    pub fn set_comment(&mut self, setter: String) {
        self.comment = setter
    }
    pub fn set_postscript(&mut self, setter: String) {
        self.postscript = setter
    }
    // pub fn set_报文种类(&mut self, setter: String) {
    //     self.报文种类 = setter
    // }
    // pub fn set_业务类型(&mut self, setter: String) {
    //     self.业务类型 = setter
    // }
    // pub fn set_业务种类(&mut self, setter: String) {
    //     self.业务种类 = setter
    // }
    // pub fn set_凭证号码(&mut self, setter: String) {
    //     self.凭证号码 = setter
    // }
    // pub fn set_收支申报号(&mut self, setter: String) {
    //     self.收支申报号 = setter
    // }
    // pub fn set_业务标识号(&mut self, setter: String) {
    //     self.业务标识号 = setter
    // }
    // pub fn set_业务编号(&mut self, setter: String) {
    //     self.业务编号 = setter
    // }
    // pub fn set_发起行行号(&mut self, setter: String) {
    //     self.发起行行号 = setter
    // }
    // pub fn set_接收行行号(&mut self, setter: String) {
    //     self.接收行行号 = setter
    // }
    // pub fn set_发起行名称(&mut self, setter: String) {
    //     self.发起行名称 = setter
    // }
    // pub fn set_接收行名称(&mut self, setter: String) {
    //     self.接收行名称 = setter
    // }
    // pub fn set_入账账号(&mut self, setter: String) {
    //     self.入账账号 = setter
    // }
    // pub fn set_入账户名(&mut self, setter: String) {
    //     self.入账户名 = setter
    // }
    // pub fn set_交易机构(&mut self, setter: String) {
    //     self.交易机构 = setter
    // }
    // pub fn set_交易渠道(&mut self, setter: String) {
    //     self.交易渠道 = setter
    // }
    // pub fn set_交易流水号(&mut self, setter: String) {
    //     self.交易流水号 = setter
    // }
    // pub fn set_经办(&mut self, setter: String) {
    //     self.经办 = setter
    // }
    // pub fn set_回单编号(&mut self, setter: String) {
    //     self.回单编号 = setter
    // }
    // pub fn set_回单验证码(&mut self, setter: String) {
    //     self.回单验证码 = setter
    // }
    // pub fn set_打印时间(&mut self, setter: String) {
    //     self.打印时间 = setter
    // }
    // pub fn set_打印次数(&mut self, setter: String) {
    //     self.打印次数 = setter
    // }
    // pub fn get_客户号(&self) -> &String {
    //     &self.客户号
    // }
    pub fn get_id(&self) -> &u32 {
        &self.id
    }
    pub fn get_date(&self) -> &String {
        &self.date
    }
    pub fn get_from_id(&self) -> &String {
        &self.from_id
    }
    pub fn get_收款人名称(&self) -> &String {
        &self.name
    }
    pub fn get_from_name(&self) -> &String {
        &self.from_name
    }
    // pub fn get_收款人开户行(&self) -> &String {
    //     &self.收款人开户行
    // }
    // pub fn get_付款人开户行(&self) -> &String {
    //     &self.付款人开户行
    // }
    pub fn get_ammount(&self) -> &f64 {
        &self.ammount
    }
    // pub fn get_金额大写(&self) -> &String {
    //     &self.金额大写
    // }
    pub fn get_usage(&self) -> &String {
        &self.usage
    }
    pub fn get_comment(&self) -> &String {
        &self.comment
    }
    pub fn get_postscript(&self) -> &String {
        &self.postscript
    }
    // pub fn get_报文种类(&self) -> &String {
    //     &self.报文种类
    // }
    // pub fn get_业务类型(&self) -> &String {
    //     &self.业务类型
    // }
    // pub fn get_业务种类(&self) -> &String {
    //     &self.业务种类
    // }
    // pub fn get_凭证号码(&self) -> &String {
    //     &self.凭证号码
    // }
    // pub fn get_收支申报号(&self) -> &String {
    //     &self.收支申报号
    // }
    // pub fn get_业务标识号(&self) -> &String {
    //     &self.业务标识号
    // }
    // pub fn get_业务编号(&self) -> &String {
    //     &self.业务编号
    // }
    // pub fn get_发起行行号(&self) -> &String {
    //     &self.发起行行号
    // }
    // pub fn get_接收行行号(&self) -> &String {
    //     &self.接收行行号
    // }
    // pub fn get_发起行名称(&self) -> &String {
    //     &self.发起行名称
    // }
    // pub fn get_接收行名称(&self) -> &String {
    //     &self.接收行名称
    // }
    // pub fn get_入账账号(&self) -> &String {
    //     &self.入账账号
    // }
    // pub fn get_入账户名(&self) -> &String {
    //     &self.入账户名
    // }
    // pub fn get_交易机构(&self) -> &String {
    //     &self.交易机构
    // }
    // pub fn get_交易渠道(&self) -> &String {
    //     &self.交易渠道
    // }
    // pub fn get_交易流水号(&self) -> &String {
    //     &self.交易流水号
    // }
    // pub fn get_经办(&self) -> &String {
    //     &self.经办
    // }
    // pub fn get_回单编号(&self) -> &String {
    //     &self.回单编号
    // }
    // pub fn get_回单验证码(&self) -> &String {
    //     &self.回单验证码
    // }
    // pub fn get_打印时间(&self) -> &String {
    //     &self.打印时间
    // }
    // pub fn get_打印次数(&self) -> &String {
    //     &self.打印次数
    // }
}
impl<'a> ToRow for &'a Incoming {
    fn to_row(&self) -> Row {
        row![
            self.date.as_ref(),
            self.id as f64,
            self.from_id.as_ref(),
            self.name.as_ref(),
            self.from_name.as_ref(),
            self.ammount,
            self.usage.as_ref(),
            self.comment.as_ref(),
            self.postscript.as_ref()
        ]
    }
}
