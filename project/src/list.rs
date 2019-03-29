use std::thread;
use std::collections::LinkedList;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut list: LinkedList<u64> = LinkedList::new();

    thread::spawn(move || {
        add_elements_to(&mut list);
        tx.send(list).unwrap();
    }).join().unwrap();

    let list = rx.recv().unwrap();
    println!("{:?}", &list);
}

fn add_elements_to(list: &mut LinkedList<u64>) {
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
}

