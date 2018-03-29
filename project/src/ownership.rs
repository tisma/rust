fn main() {
    let a = [1, 2, 3];
    let b = a;
    println!("{:?} {:?}", a, b);

    let mut aa = vec![1, 2, 3];
    let bb = &mut aa;
    println!("{:?} {:?}", aa, bb);
}