extern crate rayon;

use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let sum: i64 = line.par_split_whitespace().map(|word| word.parse::<i64>().unwrap()).sum();
    println!("Sum: {}", sum);
}

