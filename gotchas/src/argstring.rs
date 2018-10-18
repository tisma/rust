fn main() {
    for arg in std::env::args().skip(1) {
        respond(&arg)
    }
}

fn respond(arg: &str) {
    match arg {
        "hi" => println!("Hello there!"),
        "bye" => println!("Ok, goodbye!"),
        _ => println!("Sorry, I don't know what {} means", arg),
    }
}

