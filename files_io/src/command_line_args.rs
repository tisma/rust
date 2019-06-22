use std::env;

fn main() {
    let mut sum = 0;

    for input in env::args() {
        let _i = match input.parse::<i32>() {
            Ok(_i) => {
                sum += _i
            },
            Err(_e) => {
                println!("{}: Not a valid integer!", input)
            }
        };
        println!("Sum: {}", sum);
    }
}