use std::error::Error;
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use calamine::{Reader, Xlsx, open_workbook, DataType};

type Contents = HashMap<Vec<String>, String>;

pub fn read_file(filename: &str, sheet: &str, column: usize) -> Result<Contents, Box<dyn Error>> {
    let mut excel: Xlsx<_> = open_workbook(filename)?;

    Ok(parse_data(&mut excel, sheet, column))
}

fn parse_data(excel: &mut Xlsx<BufReader<File>>, sheet: &str, column: usize) -> Contents {
    let mut contents = HashMap::new();
    
    if let Some(Ok(r)) = excel.worksheet_range(sheet) {
        for (i, row) in r.rows().enumerate() {
            if i == 0 {
                continue;
            }

            if let DataType::String(key) = &row[0] {
                match &row[column] {
                    DataType::String(val) => {
                        let keys = key
                            .split(".")
                            .map(|v| v.to_string())
                            .collect();

                        contents.insert(keys, val.to_string());
                    },
                    _ => (),
                }
            }
        }
    }

    contents
}
