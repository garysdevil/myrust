// clap宝箱提供命令行输入参数解析功能
// 练习clap宝箱的使用

#![allow(dead_code)]

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long)]
    one: String, // String类型的参数必须输入，否则会报错
    #[clap(long)]
    two: bool, // bool类型的参数默认值为false
    #[clap(default_value = "33", long)] // default_value = "33" 定义默认值为“33”
    three: String,
    #[clap(long, hide = true)] // hide = true 定义在--help时不显示帮助
    pub four: Option<String>, // 使用Option<>，则这个变量可以为None，在输入参数时可以不进行输入，否则启动时不输入这个参数会报错
}

pub fn main() {
    let cli = Cli::parse();

    println!("one: {:?}", cli.one);
    println!("two: {:?}", cli.two);
}
