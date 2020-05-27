use clap;

#[derive(Debug)]
pub struct Config {
    pub input: String,
    pub output: String,
    pub separator: String,
    pub column: usize,
    pub sheet: String,
}

pub fn parse_config(matches: clap::ArgMatches) -> Config {
    let column = matches.value_of("column").unwrap();
    let column = column.parse::<usize>().unwrap();

    Config {
        output: matches.value_of("output").unwrap().to_string(),
        input: matches.value_of("input").unwrap().to_string(),
        separator: matches.value_of("separator").unwrap().to_string(),
        column,
        sheet: matches.value_of("sheet").unwrap().to_string(),
    }
}
