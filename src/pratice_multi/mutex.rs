#![allow(dead_code)]

use std::ops::Sub;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Instant;

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

fn add_n_times(n: u64, counter: Arc<Mutex<u64>>) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }
    })
}
pub fn main1() {
    let s = Instant::now();

    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        let counter = counter.clone();
        handles.push(add_n_times(N_TIMES, counter));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, *counter.lock().unwrap());

    println!("mutex.rs main1 {:?}", Instant::now().sub(s));
}

// 所有操作放在一个函数里
pub fn main2() {
    let s = Instant::now();

    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..N_TIMES {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, *counter.lock().unwrap());

    println!("mutex.rs main2 {:?}", Instant::now().sub(s));
}
