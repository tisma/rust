fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    {
        let x = 5;
        let y = x;
        println!("{},{}", x, y);

        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("{},{}", s1, s2);
    }

    {
        let mut s1 = String::from("Hello");
        let len = calculate_length(&s1);
        println!("Length of {} is {}", s1, len);

        change_string(&mut s1);
        let len = calculate_length(&s1);
        println!("Length of {} is {}", s1, len);

        println!("{}", no_dangle_reference());
    }

    println!("{}", first_word_slice(&s));

    {
        let s = String::from("hello world");
        let hello = &s[..5];
        let world = &s[6..11];
        println!("{} {}", hello, world);
    }
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if b' ' == item {
            return &s[0..i]
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn no_dangle_reference() -> String {
    let s = String::from("hello");
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world")
}