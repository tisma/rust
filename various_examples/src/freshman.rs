fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut sum = 0;
    for word in line.split_whitespace() {
        let num = word.parse::<i64>().unwrap();
        sum += num;
    }
    println!("Sum: {}", sum);
}

