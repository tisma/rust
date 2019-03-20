macro_rules! hello_world {
    () => {
        println!("Hello world!")
    };
}

fn double(a: i32) -> i32 {
    return a + a
}

fn main() {
    let my_name = "Mihalis";
    let salute = format!("Hello {}!", my_name);
    println!("{}", salute);

    hello_world!();
    
    assert_eq!(double(12), 24);
    assert_eq!(double(12), 26);
}

