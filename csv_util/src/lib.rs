use std::io::{Read, BufReader, BufRead};
use std::collections::HashMap;

pub struct Reader {
    data: HashMap<String, Vec<String>>,
}

impl Reader {
    pub fn print_data(&self) {
        println!("{:?}", self.data);
    }
}

pub struct ReaderBuilder {
    delimiter: u8,
}

impl ReaderBuilder {
    pub fn new() -> Self {
        Self {
            delimiter: b',',
        }
    }

    pub fn delimiter(&mut self, delimiter: u8) -> &mut Self {
        self.delimiter = delimiter;
        self
    }

    pub fn parse<R: Read>(&self, reader: R) -> std::io::Result<Reader> {
        let mut data = Vec::new();
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();

        while buf_reader.read_line(&mut line)? > 0 {
            let row: Vec<String> = line.trim_end().
                split(self.delimiter as char).
                map(|s| s.to_string()).
                collect();
            data.push(row);
            line.clear();
        }

        let mut processed: HashMap<String, Vec<String>> = HashMap::new();

        for row in data.iter().skip(1) {
            for (j, val) in row.iter().enumerate() {
                let key = &data[0][j];
                processed.entry(key.clone()).or_default().push(val.clone());
            }
        }

        Ok( Reader { data: processed } )
    }
}


pub fn parse_args(args: &[String]) -> (&str, &str, &str) {
    if args.len() < 3 {
        panic!("Not enough arguments");
    }

    let file_path = &args[1];
    let query = &args[2];

    let (column, value) = query.split_once("=").unwrap();

    (file_path, column, value)
}