use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long)]
    two: String,
    #[clap(long)]
    one: String,
    #[clap(long)]
    pub three: Option<String>, // 使用Option<>，则这个变量可以为None，在输入参数时可以不进行输入
}

pub fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}