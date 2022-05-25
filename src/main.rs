mod local;
mod pratice;

mod root;

fn main() {
    println!("I am the main() function");
    // root::root_fn();
    // pratice::pratice_1::main();
    // pratice::pratice_1::main(); // cargo run --features feature1
    // pratice::pratice_5::main(); // cargo run --features feature2

    local::local_fn(); // 运行 local.rs 文件里的local_fn方法，此文件只存在于本地，不上传进代码仓库。
    
}
