mod local_1;
mod local_2;
mod pratice;
mod root;

fn main() {
    println!("I am the main() function");
    // root::root_fn();
    // pratice::pratice_1::main();

    // #[cfg(feature = "feature1")]
    // pratice::tokio_1::main(); // cargo run --features feature1
    // pratice::tokio_2::main(); // cargo run --features feature1

    // #[cfg(feature = "feature2")]
    // pratice::macro_procedural::main(); // cargo run --features feature2

    local_1::local_fn(); // 运行 local1.rs 文件里的local_fn方法，此文件只存在于本地，不上传进代码仓库。
}
