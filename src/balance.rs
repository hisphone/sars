use std::{convert::Infallible, path::Path, str::FromStr};

use crate::from_txt::{FromTxt, TxtReader};

#[derive(Debug, Default)]
pub struct Balances(Vec<Balance>);

impl AsRef<Vec<Balance>> for Balances {
    fn as_ref(&self) -> &Vec<Balance> {
        &self.0
    }
}

impl AsMut<Vec<Balance>> for Balances {
    fn as_mut(&mut self) -> &mut Vec<Balance> {
        &mut self.0
    }
}

impl<P> TxtReader<P> for Balances where P: AsRef<Path> {}

impl<P> FromTxt<P> for Balances where P: AsRef<Path> {}

impl FromStr for Balances {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Balance {
    虚拟账号: String,
    虚拟账户名称: String,
    自有额度: f64,
}
