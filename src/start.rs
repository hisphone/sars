use crate::from_file::FromExcel;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::path::Path;

#[derive(Debug, Default)]
pub struct Starts(Vec<Start>);
impl AsRef<Vec<Start>> for Starts {
    fn as_ref(&self) -> &Vec<Start> {
        &self.0
    }
}

impl AsMut<Vec<Start>> for Starts {
    fn as_mut(&mut self) -> &mut Vec<Start> {
        &mut self.0
    }
}
impl<P> FromExcel<P> for Starts
where
    P: AsRef<Path>,
{
    fn from_excel(path: P) -> anyhow::Result<Self> {
        let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
        let worksheet = workbook
            .worksheet_range_at(0)
            .ok_or(anyhow::Error::msg("range err"))??;
        let mut starts = Starts::default();
        for row in worksheet
            .rows()
            .skip(5)
            .map(|row| (&row[0], &row[1], &row[5]))
        {
            match row {
                (DataType::String(id), DataType::String(name), DataType::Float(start)) => {
                    let start = Start::new(id.parse().unwrap(), name.to_string(), *start);
                    starts.as_mut().push(start);
                }
                (DataType::String(id), DataType::String(name), DataType::Empty) => {
                    let start = Start::new(id.parse().unwrap(), name.to_string(), 0.0);
                    starts.as_mut().push(start);
                }
                _ => println!("{row:?}"),
            }
        }

        Ok(starts)
    }
}
#[derive(Debug, Default)]
pub struct Start {
    id: u32,
    name: String,
    start: f64,
}
impl Start {
    pub fn new(id: u32, name: String, start: f64) -> Self {
        Self { id, name, start }
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_start(&self) -> f64 {
        self.start
    }
}
