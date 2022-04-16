use anyhow::{Context, Result};
use calamine::{open_workbook_auto, RangeDeserializerBuilder, Reader};
use encoding::{all::GB18030, DecoderTrap, Encoding};
use serde::Deserialize;
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

pub trait FromExcel<P>
where
    P: AsRef<Path>,
    Self: Sized,
{
    fn from_excel(path: P) -> anyhow::Result<Self>;
}

pub trait FromFormatedExcel<'a, P>
where
    P: AsRef<Path>,
    Self: Sized + IntoIterator + From<Vec<<Self as IntoIterator>::Item>>,
{
    fn from_formated_excel(path: P) -> anyhow::Result<Self>
    where
        for<'de> <Self as IntoIterator>::Item: Deserialize<'de>,
    {
        let mut workbook = open_workbook_auto(path).unwrap();
        let worksheet = workbook
            .worksheet_range_at(0)
            .ok_or(anyhow::Error::msg("range err"))??;
        let iter = RangeDeserializerBuilder::new()
            .has_headers(true)
            .from_range::<_, Self::Item>(&worksheet)?
            .filter_map(|res| res.ok())
            .collect::<Vec<_>>();

        Ok(Self::from(iter))
    }
}
