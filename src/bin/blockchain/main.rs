use myblockchain;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let mut mybc = myblockchain::blockchain::BlockChain::new_blockchain();

    mybc.add_block(String::from("0x000 -> 0x110 10 BTC"));
    thread::sleep(Duration::from_secs(2));
    mybc.add_block(String::from("0x000 -> 0x111 5 BTC"));

    for b in mybc.blocks {
        println!("{:?}", b);
    }
}
