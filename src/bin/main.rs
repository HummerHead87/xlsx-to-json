use std::fs;
use xlsx_json::xlsx;
use xlsx_json::object::Object;
use clap::{App, Arg};

fn main() {
    let contents = xlsx::read_file("../test_s.xlsx", "Dictionary");
    
    let contents = Object::from(&contents);

    fs::write("test.json", serde_json::to_string(contents.data()).unwrap()).unwrap();
}
