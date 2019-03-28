use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Program(&'static str),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Error {
        Error::Program(e)
    }
}

fn load_csv(filename: &str) -> Result<String, Error> {
    let mut f = File::open(filename)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        return Err("input file missing")?
    }

    Ok(buffer)
}

fn replace_column(data: String, column: &str, replacement: &str) -> Result<String, Error> {
    let mut lines = data.lines();

    let columns = lines.next().unwrap();
    let columns: Vec<&str> = columns.split(',').collect();
    let column_number = columns.iter().position(|&e| e == column);
    let column_number = match column_number {
        Some(column) => column,
        None => Err("column name doesn't exist in the input file")?
    };

    let mut result = String::with_capacity(data.capacity());
    result.push_str(&columns.join(","));
    result.push('\n');

    for line in lines {
        let mut records: Vec<&str> = line.split(',').collect();
        records[column_number] = replacement;
        result.push_str(&records.join(","));
        result.push('\n');
    }

    Ok(result)
}

fn write_csv(data: &str, filename: &str) -> Result<(), Error> {
    let mut buffer = File::create(filename)?;

    buffer.write_all(data.as_bytes())?;

    Ok(())
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();

    let filename = args.next().unwrap();
    let column_name = args.next().unwrap();
    let replacement = args.next().unwrap();
    let output_filename = args.next().unwrap();

    let csv_data = load_csv(&filename).unwrap();
    let modified_data = replace_column(csv_data, &column_name, &replacement).unwrap();
    write_csv(&modified_data, &output_filename).unwrap();
}
