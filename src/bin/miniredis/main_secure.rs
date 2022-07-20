use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use bytes::Bytes;
// type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    // 线程安全的分片数据库
    let shared_db = new_sharded_db(3);
    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        println!("Accepted");

        // Clone the handle to the hash map.
        let shared_db = shared_db.clone();

        // process(socket).await; // 通过tokio::spawn开启一个任务进行具体的操作，从而避免阻塞
        tokio::spawn(async move {
            process(socket, shared_db).await;
        });
    }
}

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut shared_db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        shared_db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(shared_db)
}

use mini_redis::Command::{self, Get, Set};
// type Db = Arc<Mutex<HashMap<String, Bytes>>>;

async fn process(socket: TcpStream, shared_db: Arc<Vec<Mutex<HashMap<String, Bytes>>>>) {
    
    // let mut shared_db = HashMap::new();

    // The `Connection` lets us read/write redis **frames** instead of byte streams. 
    // The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    // 使用 `read_frame` 接收 connection 发送过来的指令
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame.clone());
        let response = match Command::from_frame(frame).unwrap() { // 获取帧对应的指令
            Set(cmd) => {
                // let mut shared_db = shared_db.lock().unwrap();
                // shared_db.insert(cmd.key().to_string(), cmd.value().clone()); // The value is stored as `bytes::Bytes`

                let hash = i64::from_str_radix(myutils::util_cryto::get_hash(cmd.key().as_bytes()).as_str(), 16).unwrap() as usize; //.parse::<usize>().unwrap(); // 报错

                let len = shared_db.len();
                let shared_seq = hash % len;
                let mut shard = shared_db.get(shared_seq).unwrap().lock().unwrap();
                shard.insert(cmd.key().to_string(), cmd.value().clone()); 
                
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                // let shared_db = shared_db.lock().unwrap();
                let hash = myutils::util_cryto::get_hash(cmd.key().as_bytes()).parse::<usize>().unwrap();
                let len = shared_db.len();
                let shared_seq = hash % len;
                let shard = shared_db.get(shared_seq).unwrap().lock().unwrap();

                if let Some(value) = shard.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
