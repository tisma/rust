fn main() {
    let v = vec![1, 2, 3, 4];
    let sum: i32 = v.iter().map(|x| x*x).sum();
    println!("{}", sum);
}

