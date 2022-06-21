// 命令行输入参数

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
    #[clap(hide = true, long)] // hide = true 定义在--help时不显示帮助
    four: String,
    #[clap(long)]
    pub five: Option<String>, // 使用Option<>，则这个变量可以为None，在输入参数时可以不进行输入，否则启动时不输入这个参数会报错
}

pub fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}