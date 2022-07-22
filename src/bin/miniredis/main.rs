use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use mini_redis::Command::{self, Get, Set};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use bytes::Bytes;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        println!("Accepted");

        // Clone the handle to the hash map.
        let db = db.clone();

        // process(socket).await; // 通过tokio::spawn开启一个任务进行具体的操作，从而避免阻塞
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}



type Db = Arc<Mutex<HashMap<String, Bytes>>>;
async fn process(socket: TcpStream, db: Db) {
    // The `Connection` lets us read/write redis **frames** instead of byte streams. 
    // The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    // 使用 `read_frame` 接收 connection 发送过来的指令
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame.clone());
        let response = match Command::from_frame(frame).unwrap() { // 获取帧对应的指令
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone()); // The value is stored as `bytes::Bytes`
                
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
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