use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("/tmp/file.txt")?;
    let buffer = "Hello Linux Journal!\n";
    file.write_all(buffer.as_bytes())?;
    println!("Finish writing...");

    let mut input = File::open("/tmp/file.txt")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;
    print!("Read: {}", input_buffer);
    Ok(())
}

