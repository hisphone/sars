use crate::from_file::FromExcel;
use calamine::{open_workbook, DataType, Reader, Xlsx};
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
                    DataType::String(id),
                    DataType::String(name),
                    DataType::String(outgoing),
                ) => {
                    temp = date.to_string();
                    let outgoing = Outgoing::new(
                        id.parse().unwrap(),
                        name.to_string(),
                        temp.to_string(),
                        outgoing.parse().unwrap(),
                    );
                    outgoings.as_mut().push(outgoing);
                }
                (
                    DataType::Empty,
                    DataType::String(id),
                    DataType::String(name),
                    DataType::String(outgoing),
                ) => {
                    let outgoing = Outgoing::new(
                        id.parse().unwrap(),
                        name.to_string(),
                        temp.to_string(),
                        outgoing.parse().unwrap(),
                    );
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
