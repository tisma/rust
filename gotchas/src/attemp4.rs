use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::num::ParseIntError;
use std::result;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Parse(ParseIntError),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref error) => error.fmt(formatter),
            Error::Parse(ref error) => error.fmt(formatter),
        }
    }
}

impl error::Error for Error {
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::Parse(error)
    }
}

type Result<T> = result::Result<T, Error>;

fn main() -> Result<()> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let mut sum = 0;
    for word in line.split_whitespace() {
        let num = word.parse::<i64>()?;
        sum += num;
    }
    println!("Sum: {}", sum);
    Ok(())
}
