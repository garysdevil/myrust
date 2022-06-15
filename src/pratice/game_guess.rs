#![allow(dead_code)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn main() {
    println!("Game: start to guess the lucky number!");

    let lucky_number: u32 = rand::thread_rng().gen_range(0..100);

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
