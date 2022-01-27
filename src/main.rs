mod local;
mod pratice;

mod root;

fn main() {
    println!("I am the main() function");
    // root::root_fn();
    // pratice::pratice_1::main();
    // pratice::pratice_1::main(); // cargo run --features feature1
    // pratice::pratice_5::main(); // cargo run --features feature2

    local::local_fn();
}
