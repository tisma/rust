fn fib(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    for _ in 1..n {
        let t = a;
        a += b;
        b = t;
    }
    return a;
}

#[test]
fn fib_values() {
    let expected = vec![1, 1, 2, 3, 5, 8, 13];

    let actual = (0..7).map(fib).collect::<Vec<_>>();

    assert_eq!(actual, expected);
}

fn main() {
    println!("Hello, world!");
}
