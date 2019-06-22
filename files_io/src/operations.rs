use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("/tmp/test.txt")?;
    let buffer = "Hello Linux Journal!\n";
    file.write_all(buffer.as_bytes())?;
    println!("Finish writing...");

    fs::rename("/tmp/test.txt", "/tmp/LJ.txt")?;
    fs::remove_file("/tmp/LJ.txt")?;
    println!("Finish deleting...");

    Ok(())
}
