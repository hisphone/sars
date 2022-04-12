use anyhow::{Context, Result};
use encoding::{all::GB18030, DecoderTrap, Encoding};
use std::{io::Read, path::Path, str::FromStr};

pub trait TxtReader<P: AsRef<Path>> {
    fn auto_read(path: P) -> Result<String> {
        Self::utf8_read(&path)
            .or_else(|_| Self::gb18030_read(&path))
            .with_context(|| "ERROR:未知编码方式")
    }
    fn utf8_read(path: &P) -> Result<String> {
        let mut file = std::fs::File::open(path)?;
        let mut context = String::new();
        file.read_to_string(&mut context)?;
        Ok(context)
    }
    fn gb18030_read(path: &P) -> Result<String> {
        let mut file = std::fs::File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        GB18030
            .decode(&buf, DecoderTrap::Strict)
            .map_err(|_| anyhow::Error::msg("ERROR:GB18030读取失败"))
    }
}

pub trait FromTxt<P>: TxtReader<P>
where
    P: AsRef<Path>,
    Self: FromStr,
    <Self as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    fn from_txt(path: P) -> Result<Self> {
        Ok(Self::from_str(&Self::auto_read(path)?)?)
    }
}

// #[test]
// fn test_txt_reader() {
//     #[derive(Debug, PartialEq)]
//     struct Test(String);
//     impl<P: AsRef<Path>> TxtReader<P> for Test {}
//     impl<P: AsRef<Path>> FromTxt<P> for Test {
//         fn from_str(context: &str) -> Result<Self> {
//             Ok(Self(context.to_string()))
//         }
//     }
//     let context = Test::from_txt("readme.md");
//     assert_eq!(
//         context.unwrap(),
//         Test("\"# sars\"\r\nsub_account_reconcile_system\r\n".to_string())
//     )
// }
