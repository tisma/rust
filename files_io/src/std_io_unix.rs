use std::io::Write;
use std::io;

fn main() {
    println!("Please give me your name:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            print!("Your name is {}", input);
        }
        Err(error) => println!("error: {}", error)
    }

    let mut stderr = std::io::stderr();
    writeln!(&mut stderr, "This is an error message!").unwrap();
    eprintln!("That is another error message!")
}
