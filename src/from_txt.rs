use anyhow::Result;
use encoding::{all::GB18030, DecoderTrap, Encoding};
use std::{io::Read, path::Path};

pub trait TxtReader<P: AsRef<Path>> {
    fn auto_read(path: P) -> Result<String> {
        println!("开始自动读取，{:?}", path.as_ref());
        Self::utf8_read(&path).or_else(|_| {
            println!("utf8解码失败");
            Self::gb18030_read(&path)
        })
    }
    fn utf8_read(path: &P) -> Result<String> {
        println!("开始按utf-8解码，{:?}", path.as_ref());
        let mut file = std::fs::File::open(path)?;
        let mut context = String::new();
        file.read_to_string(&mut context)?;
        Ok(context)
    }
    fn gb18030_read(path: &P) -> Result<String> {
        println!("开始按GB18030解码，{:?}", path.as_ref());
        let mut file = std::fs::File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        GB18030
            .decode(&buf, DecoderTrap::Strict)
            .map_err(|_| anyhow::Error::msg("ERROR:未知编码方式"))
    }
}
pub trait FromTxt<'a, P: AsRef<Path>>: TxtReader<P>
where
    Self: Sized + Default,
{
    type Filter;
    fn from_str(context: String) -> Result<Self> {
        let mut target = Self::default();
        context
            .lines()
            .rev()
            .for_each(|line| match Self::filter(line) {
                Filter => Self::handler(Self::filter(line), &mut target),
            });
        Ok(target)
    }
    fn filter(line: &str) -> Self::Filter;
    fn handler(filter: Self::Filter, target: &mut Self);
    fn from_txt(path: P) -> Result<Self> {
        Ok(Self::from_str(Self::auto_read(path)?)?)
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
