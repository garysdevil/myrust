use std::sync::{Arc, Mutex};
use std::thread;
const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

fn main2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let n = N_TIMES * N_THREADS;
    for _ in 0..n {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
