// 实现一个简单的grep功能的程序，可以根据文件搜索内容，可以根据是否有环境变量CASE_INSENSITIVE，决定是否是大小写敏感的。
// 运行方式如下
// cargo run 需要搜索的字符串 文件路径
// CASE_INSENSITIVE=1 cargo run 需要搜索的字符串 文件路径

// 这个版本通过闭包和迭代器让代码变得更简洁，也更符合函数式编程。

use std::env;
use std::process;

mod minigrep;
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    // 使用迭代器来替代引用
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
