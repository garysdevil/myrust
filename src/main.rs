mod local1;
mod local2;
mod pratice;
mod test;
mod root;

fn main() {
    println!("I am the main() function");
    // root::root_fn();
    // pratice::pratice_1::main();
    // pratice::pratice_2::main(); // cargo run --features feature1
    // pratice::pratice_5::main(); // cargo run --features feature2

    local1::local_fn(); // 运行 local1.rs 文件里的local_fn方法，此文件只存在于本地，不上传进代码仓库。
    
}
