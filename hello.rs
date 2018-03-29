fn main() {
    println!("{}, {}!", "Hello", "world");
    println!("{0}, {1}!", "Hello", "world");
    println!("{greeting}, {name}!", greeting="Hello", name="world");

    println!("{:?}", [1,2,3]);
    println!("{:#?}", [1,2,3]);

    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x);
}

