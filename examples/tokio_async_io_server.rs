// 使用 AsyncReadExt::read 和 AsyncWriteExt::write_all 实现数据拷贝:

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                // 从socket读取
                match socket.read(&mut buf).await {
                    // 返回 Ok(0) 表示 socket 连接已断开
                    Ok(0) => return,
                    Ok(n) => {
                        // 数据写入
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // 错误返回
                            return;
                        }
                    }
                    Err(_) => {
                        // 处理错误
                        return;
                    }
                }
            }
        });
    }
}