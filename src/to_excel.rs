use simple_excel_writer::{Row, Workbook};

pub trait Header {
    fn header(&self) -> Row;
}
pub trait ToRow {
    fn to_row(&self) -> Row;
}
pub trait Title {
    fn title(&self) -> &'static str;
}
pub trait IntoExcel: Header + Title + IntoIterator
where
    <Self as IntoIterator>::Item: ToRow,
    Self: Sized,
{
    fn into_excel(self, filename: &str) {
        let mut wb = Workbook::create(
            std::env::current_exe()
                .unwrap()
                .with_file_name(filename)
                .to_str()
                .unwrap(),
        );
        let mut sheet_base = wb.create_sheet(if Self::title(&self).is_empty() {
            "sheet1"
        } else {
            Self::title(&self)
        });
        wb.write_sheet(&mut sheet_base, move |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(self.header())?;
            for data in self {
                sw.append_row(data.to_row())?;
            }
            Ok(())
        })
        .unwrap();
        wb.close().expect("close excel error!");
    }
}
