use std::env;
use std::fs;
use csv_util::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (file_path, column, value) = parse_args(&args);

    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        panic!("Error reading file {}: {}", file_path, err);
    });

    let mut builder = ReaderBuilder::new();

    let rdr = match builder.delimiter(b'\t').parse(contents.as_bytes()) {
        Ok(rdr) => rdr,
        Err(e) => panic!("Error parsing file {}: {}", file_path, e),
    };

    rdr.print_data();

    println!("column: {}; value: {}", column, value);

    if rdr.search(column.to_string(), value.to_string()) {
        println!("Found");
    } else {
        println!("Not Found");
    }
}

