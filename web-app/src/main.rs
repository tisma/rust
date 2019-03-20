extern crate reqwest;

fn fetch_text(url: &str) -> Result<(String), reqwest::Error> {
    let body = reqwest::get(url)?
        .text()?;

    Ok(body)
}

fn main() {
    let result = fetch_text("https://www.rust-lang.org");
    let count = result.unwrap();
    println!("{}", count);
}
