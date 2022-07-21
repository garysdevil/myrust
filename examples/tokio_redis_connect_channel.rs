use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use bytes::Bytes;
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

// 定义枚举，作为消息内容
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    }
}


#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    // 生成两个任务，一个用于获取 key，一个用于设置 key
    // 生成一个任务管理者，redis命令的执行者
    let t1 = task_redis_get(tx);
    let t2 = task_redis_set(tx2);
    let manager = manager(rx);

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

}

fn task_redis_get(tx: tokio::sync::mpsc::Sender<Command>) -> tokio::task::JoinHandle<()>{
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };

        // 发送 GET 请求
        tx.send(cmd).await.unwrap();

        // 等待回复
        let res = resp_rx.await;
        println!("GOT = {:?}", res);

    });
    t1
}

fn task_redis_set(tx2: tokio::sync::mpsc::Sender<Command>) -> tokio::task::JoinHandle<()>{
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // 发送 SET 请求
        tx2.send(cmd).await.unwrap();

        // 等待回复
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });
    t2
}

// 消息的接受者，redis命令的执行者
fn manager(mut rx: tokio::sync::mpsc::Receiver<Command>) -> tokio::task::JoinHandle<()>{
    // 将消息通道接收者 rx 的所有权转移到管理任务中
    let manager = tokio::spawn(async move {
        // 建立到 redis 服务器的连接
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收消息
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key , resp} => {
                    let res = client.get(&key).await;
                    // 忽略错误
                    let _ = resp.send(res);

                }
                Set { key, val ,resp} => {
                    let res = client.set(&key, val).await;
                    // 忽略错误
                    let _ = resp.send(res);
                }
            }
        }
    });
    manager
}