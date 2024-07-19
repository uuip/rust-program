use std::path::PathBuf;

use chrono::Local;
use ndarray::prelude::*;


fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let n_row = 1_00_0000;
    let n_column = 50;
    let now = Local::now();
    println!("data 准备开始，开始输出{}", now);

    let mut rng = fastrand::Rng::new();
    let data = Array2::from_shape_simple_fn((n_column, n_row), || rng.f64());
    let data = data.t();
    // println!("{:?}", data);

    println!("data 准备完成，开始输出{}", Local::now());
    let filename = "example.xlsx";
    if PathBuf::from(filename).exists() {
        std::fs::remove_file(filename)?;
    }

    {
        use simple_excel_writer::{Row, Workbook};

        let now = Local::now();
        let mut workbook = Workbook::create(filename);
        let mut sheet = workbook.create_sheet("sheet");
        workbook
            .write_sheet(&mut sheet, |sheet_writer| {
                let _ = data
                    .axis_iter(Axis(0))
                    .map(|r| sheet_writer.append_row(Row::from_iter(r.to_owned().into_iter())))
                    .collect::<Vec<_>>();
                Ok(())
            })
            .expect("write excel error!");
        workbook.close().expect("close excel error!");
        println!(
            "simple_excel_writer用时{:.2?}秒",
            Local::now().signed_duration_since(now).num_seconds()
        );
    }

    {
        use rust_xlsxwriter::*;

        let now = Local::now();
        let d = data.axis_iter(Axis(0)).map(|x| x.to_owned());
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        worksheet.write_row_matrix(0, 0, d)?;
        workbook.save(filename)?;

        println!(
            "rust_xlsxwriter用时{:.2?}秒",
            Local::now().signed_duration_since(now).num_seconds()
        );
    }

    Ok(())
}
