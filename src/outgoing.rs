use crate::{from_file::FromExcel, to_excel::ToRow};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use simple_excel_writer::{row, Row};
use std::path::Path;

#[derive(Debug, Default)]
pub struct Outgoings(Vec<Outgoing>);

impl AsRef<Vec<Outgoing>> for Outgoings {
    fn as_ref(&self) -> &Vec<Outgoing> {
        &self.0
    }
}

impl AsMut<Vec<Outgoing>> for Outgoings {
    fn as_mut(&mut self) -> &mut Vec<Outgoing> {
        &mut self.0
    }
}

impl<P> FromExcel<P> for Outgoings
where
    P: AsRef<Path>,
{
    fn from_excel(path: P) -> anyhow::Result<Self> {
        let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
        let worksheet = workbook
            .worksheet_range_at(0)
            .ok_or(anyhow::Error::msg("range err"))??;
        let mut outgoings = Outgoings::default();
        let mut temp = "".to_string();
        for row in worksheet
            .rows()
            .skip(4)
            .map(|row| (&row[4], &row[40], &row[41], &row[44]))
        {
            match row {
                (
                    DataType::String(date),
                    DataType::Float(id),
                    DataType::String(name),
                    DataType::Float(outgoing),
                ) => {
                    temp = date.to_string();
                    let outgoing =
                        Outgoing::new(*id as u32, name.to_string(), temp.to_string(), *outgoing);
                    outgoings.as_mut().push(outgoing);
                }
                (
                    DataType::Empty,
                    DataType::Float(id),
                    DataType::String(name),
                    DataType::Float(outgoing),
                ) => {
                    let outgoing =
                        Outgoing::new(*id as u32, name.to_string(), temp.to_string(), *outgoing);
                    outgoings.as_mut().push(outgoing);
                }
                _ => println!("{row:?}"),
            }
        }

        Ok(outgoings)
    }
}
#[derive(Debug, Default)]
pub struct Outgoing {
    id: u32,
    name: String,
    date: String,
    outgoing: f64,
}
impl Outgoing {
    pub fn new(id: u32, name: String, date: String, outgoing: f64) -> Self {
        Self {
            id,
            name,
            date,
            outgoing,
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
    pub fn get_outgoing(&self) -> f64 {
        self.outgoing
    }
}
impl ToRow for Outgoing {
    fn to_row(&self) -> Row {
        row![
            self.id as f64,
            self.get_name().as_str(),
            self.get_date().as_str(),
            self.outgoing
        ]
    }
}
