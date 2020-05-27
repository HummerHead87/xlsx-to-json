use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use calamine::{Reader, Xlsx, open_workbook, DataType};

pub fn read_file(filename: &str, sheet: &str) -> HashMap<Vec<String>, String> {
    let mut excel: Xlsx<_> = open_workbook(filename).unwrap();
    
    parse_data(&mut excel, sheet)
}

fn parse_data(excel: &mut Xlsx<BufReader<File>>, sheet: &str) -> HashMap<Vec<String>, String> {
    let mut contents = HashMap::new();
    
    if let Some(Ok(r)) = excel.worksheet_range(sheet) {
        for row in r.rows() {
            if let DataType::String(key) = &row[0] {
                match &row[2] {
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
