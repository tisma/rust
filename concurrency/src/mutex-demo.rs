use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut data = Vec::new();
    for _ in 0..10 {
        data.push(0);
    }

    let data = Arc::new(Mutex::new(data));

    let mut threads = Vec::new();

    for i in 0..5 {
        let data = Arc::clone(&data);
        let thread = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..100 {
                let mut data = data.lock().unwrap();
                let index = rng.gen_range(0, data.len());
                data[index] += i;
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
        
    println!("{:?}", data);
}

