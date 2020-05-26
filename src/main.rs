use calamine::{Reader, Xlsx, open_workbook, DataType};
use std::collections::HashMap;
use serde_json::{Map, Value};
use std::fs;

fn main() {
    let contents = read_file("../test_s.xlsx");
    let data = parse_data(&contents);

    fs::write("test.json", serde_json::to_string(&data.data).unwrap()).unwrap();
}

fn read_file(filename: &str) -> HashMap<Vec<String>, String> {
    let mut excel: Xlsx<_> = open_workbook(filename).unwrap();
    let mut contents = HashMap::new();
    
    if let Some(Ok(r)) = excel.worksheet_range("Dictionary") {
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

fn parse_data(contents: &HashMap<Vec<String>, String>) -> Object {
    let mut data = Object::new();
    
    for (key, val) in contents {
        data.insert(key, val);
    }

    data
}

#[derive(Debug)]
pub struct Object {
    data: Map<String, Value>,
}

impl Object {
    pub fn new() -> Object {
        Object {
            data: Map::new(),
        }
    }

    pub fn insert(&mut self, keys: &Vec<String>, val: &str) {
        insert_field(&mut self.data, keys, val);
    }
}

fn insert_field(map: &mut Map<String, Value>, keys: &Vec<String>, val: &str) {
    if keys.len() > 1 {
        match map.get_mut(keys.get(0).unwrap()) {
            Some(Value::Object(map)) => {
                insert_field(map, &keys[1..].to_vec(), val);
            },
            _ => {
                let mut child = Map::new();
                insert_field(&mut child, &keys[1..].to_vec(), val);

                let v = Value::Object(child);
                map.insert(keys.get(0).unwrap().to_string(), v);
            }
        }
    } else {
        map.insert(keys.get(0).unwrap().to_string(), Value::String(val.to_string()));
    }
}
