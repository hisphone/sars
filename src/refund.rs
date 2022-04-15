use crate::from_file::FromExcel;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::path::Path;

#[derive(Debug, Default)]

pub struct Refunds(Vec<Refund>);
impl AsRef<Vec<Refund>> for Refunds {
    fn as_ref(&self) -> &Vec<Refund> {
        &self.0
    }
}

impl AsMut<Vec<Refund>> for Refunds {
    fn as_mut(&mut self) -> &mut Vec<Refund> {
        &mut self.0
    }
}

impl<P> FromExcel<P> for Refunds
where
    P: AsRef<Path>,
{
    fn from_excel(path: P) -> anyhow::Result<Self> {
        let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
        let worksheet = workbook
            .worksheet_range_at(0)
            .ok_or(anyhow::Error::msg("range err"))??;
        let mut refunds = Refunds::default();
        let mut temp = "".to_string();
        for row in worksheet
            .rows()
            .skip(4)
            .map(|row| (&row[4], &row[40], &row[41], &row[44]))
        {
            match row {
                (
                    DataType::String(date),
                    DataType::String(id),
                    DataType::String(name),
                    DataType::String(refund),
                ) => {
                    temp = date.to_string();
                    let refund = Refund::new(
                        id.parse().unwrap(),
                        name.to_string(),
                        temp.to_string(),
                        refund.parse().unwrap(),
                    );
                    refunds.as_mut().push(refund);
                }
                (
                    DataType::Empty,
                    DataType::String(id),
                    DataType::String(name),
                    DataType::String(refund),
                ) => {
                    let refund = Refund::new(
                        id.parse().unwrap(),
                        name.to_string(),
                        temp.to_string(),
                        refund.parse().unwrap(),
                    );
                    refunds.as_mut().push(refund);
                }
                _ => println!("{row:?}"),
            }
        }

        Ok(refunds)
    }
}

#[derive(Debug, Default)]
pub struct Refund {
    id: u32,
    name: String,
    date: String,
    refund: f64,
}
impl Refund {
    pub fn new(id: u32, name: String, date: String, refund: f64) -> Self {
        Self {
            id,
            name,
            date,
            refund,
        }
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_date(&self) -> &String {
        &self.date
    }
    pub fn get_refund(&self) -> f64 {
        self.refund
    }
}
