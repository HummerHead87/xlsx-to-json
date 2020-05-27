use std::fs;
use xlsx_json::xlsx;
use xlsx_json::object::Object;
use xlsx_json::helpers;
use clap::{App, Arg};

fn main() {
    let matches = get_matches();
    let config = helpers::parse_config(matches);

    let contents = xlsx::read_file(
        &config.input,
        &config.sheet,
        config.column,
    );

    let contents = Object::from(&contents);

    fs::write(
        &config.output,
        serde_json::to_string(contents.data()).unwrap()
    ).unwrap();
}

fn get_matches<'a>() -> clap::ArgMatches<'a> {
    App::new("xlsx-to-json")
        .version("0.1.0")
        .author("HummerHead87 <snooks87@gmail.com>")
        .about("generate json from xlsx tables")
        .arg(
            Arg::with_name("input")
            .short("i")
            .long("input")
            .takes_value(true)
            .value_name("FILE")
            .help("provide a *.xlsx file to read from")
            .required(true)
        )
        .arg(
            Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .value_name("FILE")
            .default_value("output.json")
            .help("provide a *.json file name to write result")
        )
        .arg(
            Arg::with_name("separator")
            .short("sep")
            .long("separator")
            .takes_value(true)
            .default_value(".")
            .help("separator for json field names in output file")
        )
        .arg(
            Arg::with_name("column")
            .short("col")
            .long("column")
            .takes_value(true)
            .default_value("1")
            .validator(|v| {
                match v.parse::<usize>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(String::from("Must be a number")),
                }
            })
            .help("column in xlsx file with messages")
        )
        .arg(
            Arg::with_name("sheet")
            .long("sheet")
            .takes_value(true)
            .default_value("Dictionary")
            .help("Name of the sheet in xlsx file to read from")
        )
        .get_matches()
}
