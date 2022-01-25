// 多线程

use std::thread::{sleep, spawn};
use std::time::Duration;

pub fn sleep_fn() {
    for i in 1..=10 {
        println!("sleep_fn {}", i);
        sleep(Duration::from_millis(500));
    }
}

pub fn interrup_fn() {
    for i in 1..=5 {
        println!("interrup_fn {}", i);
        sleep(Duration::from_millis(1000));
    }
}

// fn main() {

//     let sleep_fn = spawn(sleep_fn); // 生成一个子线程
//     interrup_fn();
//     sleep_fn.join().unwrap(); // join方法使主线程等待当前子线程执行结束
// }