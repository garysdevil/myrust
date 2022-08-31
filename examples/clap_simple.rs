// clap宝箱提供命令行输入参数解析功能
// 练习clap宝箱的使用

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    // short 关键字使参数可以使用简写模式，默认为第一个字母 cargo run --example clap -- -a string123
    arg1: String, // String类型的参数必须输入，否则会报错
    #[clap(long)]
    arg2: bool, // bool类型的参数默认值为false
    #[clap(long, default_value = "33")] // default_value = "33" 定义默认值为“33”
    arg3: String,
    #[clap(long, hide = true)] // hide = true 定义在--help时不显示帮助
    pub arg4: Option<String>, // 使用Option<>类型，使这个变量可以为None，在输入参数时可以不进行输入，否则启动时不输入这个参数会报错
}

pub fn main() {
    let cli = Cli::parse();

    println!("arg1: {:?}", cli.arg1);
    println!("arg2: {:?}", cli.arg2);
    println!("arg3: {:?}", cli.arg3);
    println!("arg4: {:?}", cli.arg4);
}

//  运行
// cargo run --example clap -- --arg1 string123
// cargo run --example clap -- --arg1 string123 --arg2
