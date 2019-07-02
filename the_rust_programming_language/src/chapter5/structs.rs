#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let user1 = build_user(String::from("tisma.etf@gmail.com"), String::from("tisma"));
    println!("{:#?}", user1);
    println!("{:?}", user1);

    let rect1 = Rectangle { width: 30, height: 40 };
    let rect2 = Rectangle { width: 15, height: 25 };
    println!("Area of {:?} is {}", rect1, rect1.area());
    println!("rect1 can hold rect2 {:?}: {}", rect2, rect1.can_hold(&rect2));
}
