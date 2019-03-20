fn accept(s: &str) {
    println!("{}", s);
}

fn main() {
    let l_j: &str = "Linux Journal";
    let magazine: &'static str = "magazine";
    let my_str = format!("Hello {} {}!", l_j, magazine);
    println!("my_str L:{} C:{}", my_str.len(), my_str.capacity());

    for c in my_str.chars() {
        print!("{} ", c);
    }
    
    println!();

    for (i, c) in my_str.chars().enumerate() {
        print!("{}:{} ", c, i);
    }

    println!();

    let n: &str = "10";
    match n.parse::<i32>() {
        Ok(n) => println!("{} is a number!", n),
        Err(e) => println!("{} is NOT a number!", e),
    }

    let n1: &str = "10.2";
    match n1.parse::<i32>() {
        Ok(n1) => println!("{} is a number!", n1),
        Err(e) => println!("{}: {}", n1, e),
    }

    let my_str = "This is str!";
    let mut my_string = String::from("This is string!");
    accept(&my_str);
    accept(&my_string);

    println!("my_string L:{} C:{}", my_string.len(), my_string.capacity());
    my_string.push_str("OK?");
    println!("my_stirng L:{} C:{}", my_string.len(), my_string.capacity());

    let s_str: &str = &my_string[..];
    let s_string: String = s_str.to_owned();
    println!("s_string: L:{} C:{}", s_string.len(), s_string.capacity());
}

