#![allow(dead_code)]

use std::thread;
use std::time::Duration;

// Cacher范型结构体传入的参数是一个闭包函数。
// Cacher实例当第一次调用value()时，执行闭包函数进行赋值，之后再调用value()，就不会再执行闭包函数进行赋值了
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// 通过用户传入的intensity参数，判断需要给予今天的运动量。当intensity参数大于30时，根据随机数判断今天是否休息。
fn generate_workout(intensity: u32, random_number: u32) {
    // 通过闭包实现了赖加载
    // 当expensive_result闭包函数第一次被使用时，就会进行加载。
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2)); // 这里加速加载是一件很耗时的行为。
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

pub fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 这个缓存数据结构，赋值一次后就不会被改变，还需要进一步改进
// #[test]
// fn call_with_different_values() {
//     let mut c = Cacher::new(|a| a);

//     let v1 = c.value(1);
//     let v2 = c.value(2);

//     assert_eq!(v2, 2);
// }
