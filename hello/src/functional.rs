fn main() {
    println!("{}", (1..101).map(|x| x * 2).fold(0, |x, y| x + y));
    println!("{}", (1..101).fold(0, std::ops::Add::add));
}

