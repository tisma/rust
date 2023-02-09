use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use std::thread;

fn main() {
    let secondary_thread = thread::spawn(|| {
        for i in 1..5 {
            println!("Secondary Thread Prints {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..5 {
        println!("Main Thread Prints {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    secondary_thread.join().unwrap();

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("I was created in the child thread, will be sent to main thread");
        sender.send(val).unwrap();
    });

    let received = receiver.recv().unwrap();
    println!("I have received this message from the child thread: {}", received);

    let shared_state = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for _ in 0..16 {
        let shared_state = Arc::clone(&shared_state);
        let child_thread = thread::spawn(move || {
            let mut num = shared_state.lock().unwrap();
            *num += 1;
        });
        threads.push(child_thread);
    }

    for child_thread in threads {
        child_thread.join().unwrap();
    }

    println!("Result: {}", *shared_state.lock().unwrap());
}

