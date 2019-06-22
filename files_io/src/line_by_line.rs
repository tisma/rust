use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let mut total_lines = 0;
    let mut total_chars = 0;
    let mut total_uni_chars = 0;

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} text_file", args[0]);
        return;
    }

    let input_path = ::std::env::args().nth(1).unwrap();
    let file = BufReader::new(File::open(&input_path).unwrap());

    for line in file.lines() {
        total_lines += 1;
        let my_line = line.unwrap();
        total_chars = total_chars + my_line.len();
        total_uni_chars = total_uni_chars + my_line.chars().count();
    }

    println!("Lines processed: \t\t{}", total_lines);
    println!("Characters read: \t\t{}", total_chars);
    println!("Unicode characters read: \t\t{}", total_uni_chars);
}
