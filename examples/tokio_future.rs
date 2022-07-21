// 实现一个future，具有以下功能
// 1. 等待某个特定时间点的到来
// 2. 在标准输出打印文本 
// 3. 生成一个字符串

use std::future::Future;
use std::pin::Pin; // Pin 类型是在异步函数中进行借用的关键。
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str; // 关联类型 Output 是 Future 执行完成后返回的值的类型

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<&'static str>
    {
        if Instant::now() >= self.when { // 等到了特定的时间点
            println!("Hello world"); // 在标准输出打印文本 
            Poll::Ready("done") // 生成一个字符串，返回数据 "done"
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}