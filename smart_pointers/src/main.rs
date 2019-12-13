use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
struct MyBox<T> {
    a: T,
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.a
    }
}

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// #[derive(Debug)]
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<String>>, Rc<List>),
    Nil,
}

struct Employee {
    age: i32,
}

impl Drop for Employee {
    fn drop(&mut self) {
        println!("The instance of Employee is dropped: {}", self.age);
    }
}

/*
pub fn reference_counter() {
    let first = Rc::new(Cons(25, Rc::new(Cons(80, Rc::new(Nil)))));

    println!("Reference counter after the first instance is created: {}", Rc::strong_count(&first));

    let second = Cons(2, Rc::clone(&first));

    println!("Reference counter after cloned the second from first: {}", Rc::strong_count(&first));

    {
        let third = Cons(1, Rc::clone(&first));
        println!("Reference counter inside the loop: {}", Rc::strong_count(&first));
    }

    println!("Reference counter when the scope of third ended: {}", Rc::strong_count(&first));
}
*/

pub fn ref_cell() {
    let first = Rc::new(RefCell::new(String::from("Python language.")));

    let second = Rc::new(Cons(Rc::clone(&first), Rc::new(Nil)));

    // let third = Cons(Rc::new(RefCell::new(String::from("C"))), Rc::clone(&first));

    *second.borrow_mut() = String::from("Rust language");

    println!("value of second variable is : {:?}", second);
    // println!("value of third variable is : {:?}", third);
}

fn main() {
    let a = Box::new(5);
    println!("Value in Box pointer is {}", a);

    // let recursive_list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    // println!("{:?}", recursive_list);

    let instance = MyBox { a: 10 };
    println!("{}", *(instance.deref()));

    let john = Employee { age: 10 };
    let bill = Employee { age: 30 };

    println!("The john and bill instances are created.");

    // reference_counter();
    ref_cell();
}
