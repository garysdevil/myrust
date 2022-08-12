//! # My Rust
//!
//! `my_rust` is my example code when I am learning Rust languarage

// lib.rs 是默认的库宝箱的源文件
// 只能从库宝箱的源文件向myrust宝箱外暴露模块
// 只能从库宝箱的源文件生成文档

pub fn main() {
    println!("I am the main() function in lib.rs");
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = myrust::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod pratice;
pub mod pratice_multi;
