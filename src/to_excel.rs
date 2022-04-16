use anyhow::Result;
use simple_excel_writer::{Column, Row, Workbook};

pub trait Header {
    fn header(&self) -> Row;
}
pub trait ToRow {
    fn to_row(&self) -> Row;
}
pub trait SheetName {
    fn sheet_name(&self) -> &'static str;
}
pub trait Columns {
    fn column(&self) -> Vec<Column>;
}
pub trait ToExcel: Header + SheetName + Columns + IntoIterator
where
    <Self as IntoIterator>::Item: ToRow,
    Self: Sized,
{
    fn to_excel(self, filename: &str) -> Result<()> {
        {
            let mut wb = Workbook::create(filename);
            let sheet_name = if self.sheet_name().is_empty() {
                "sheet1"
            } else {
                self.sheet_name()
            };
            let mut sheet = wb.create_sheet(sheet_name);
            self.column().into_iter().for_each(|c| sheet.add_column(c));
            wb.write_sheet(&mut sheet, move |sheet_writer| {
                let sw = sheet_writer;
                sw.append_row(self.header())?;
                for item in self.into_iter() {
                    sw.append_row(item.to_row())?;
                }
                Ok(())
            })?;
            wb.close()?;
            Ok(())
        }
    }
}
