use std::env;
use std::os::unix::fs::PermissionsExt;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} file", args[0]);
    }

    let f = ::std::env::args().nth(1).unwrap();
    let metadata = std::fs::metadata(f)?;
    let perm = metadata.permissions();
    println!("{:o}", perm.mode());
    Ok(())
}
