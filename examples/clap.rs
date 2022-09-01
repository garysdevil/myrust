// clap宝箱提供命令行输入参数解析功能
// 练习clap宝箱的使用

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value = "arg111")]
    // short 关键字使参数可以使用简写模式，默认为第一个字母 cargo run --example clap -- -a string123
    arg1: String, // 非Option类型的参数必须输入或者有默认值，否则会报错
    #[clap(short = 't', long, default_value = "222")]
    arg2: bool, // bool类型的参数默认值为false
    #[clap(long, hide = true)] // hide = true 定义在--help时不显示帮助
    arg3: Option<String>, // 使用Option<>类型，使这个变量可以为None，在输入参数时可以不进行输入，否则启动时不输入这个参数会报错
    #[clap(verbatim_doc_comment)] // 自定义说明
    /// Indexes of GPUs to use (starts from 0)
    /// Specify multiple times to use multiple GPUs
    /// Example: -g 0 -g 1 -g 2
    /// Note: Pure CPU proving will be disabled as each GPU job requires one CPU thread as well
    #[clap(short = 'g', long = "cuda")]
    cuda: Option<Vec<i16>>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Subarg1 {
        #[clap(short, long, action, default_value = "dev")]
        env: String,
    },
}

pub fn main() {
    let cli = Cli::parse();

    println!("arg1: {:?}", cli.arg1);
    println!("arg2: {:?}", cli.arg2);
    println!("arg3: {:?}", cli.arg3);

    match &cli.command {
        Some(Commands::Subarg1 { env }) => {
            println!("Subarg1 env: {:?}", env);
        }
        None => {}
    }
}

//  运行
// cargo run --example clap -- --arg1 string123
// cargo run --example clap -- --arg1 string123 --arg2
// cargo run --example clap  subarg1
