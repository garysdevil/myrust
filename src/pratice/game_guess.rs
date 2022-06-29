// 从0到100猜数字的游戏
// 练习读取命令行的标准输入

#![allow(dead_code)]

use rand::Rng; // 提供随机功能
use std::cmp::Ordering; // 提供数字比较功能
use std::io; // 可以处理命令行标准输入

pub fn main() {
    println!("Game: start to guess the lucky number!");

    let lucky_number: u32 = rand::thread_rng().gen_range(0..100); // 生成一个随机数字

    loop {
        let mut guess_nunmber: String = String::new();
        // get input number
        io::stdin()
            .read_line(&mut guess_nunmber)
            .expect("Failed to read line");

        let guess_nunmber: u32 = match guess_nunmber.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };

        println!("Your guess_nunmber is {}", guess_nunmber);

        // compare guess_nunmber with lucky_number
        match guess_nunmber.cmp(&lucky_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
