// 声明宏 Declarative Macro

#![allow(dead_code)]

pub fn main() {
    macro_rules! list {
        // $x 是变量
        // :expr 是关键字语法, 表示表达式
        // * 表示零次或多次表达式匹配
        ($($x:expr), *) => {
            {
                let mut temp_vec = Vec::new();
                $(                          
                    println!("{}", $x);
                    temp_vec.push($x);
                )*  // 多次匹配会多次运行这个代码块.
                temp_vec
            }
        }
    }
    let x = list!(1,2,3);
    println!("{:?}", x)
}