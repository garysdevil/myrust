// 异步运行时 tokio

#![allow(dead_code)]

use std::future::Future;
use std::thread::sleep;
use std::time::Duration;

use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::main]
pub async fn main() {
    // This is running on a core thread.
    println!("I am in tokio_2.rs file\n");
    
    let blocking_task = tokio::task::spawn_blocking(|| {
        println!("I am blocking_task1\n");
        sleep(Duration::from_millis(1000));
        println!("I am blocking_task2\n");
    });

    let async_task = tokio::spawn(async {
        println!("I am async_task1\n");
        sleep(Duration::from_millis(1000));
        println!("I am async_task2\n");
    });

    fn_future().await;
    
    blocking_task.await.unwrap();

    async_task.await.unwrap();

    function_name();
}

// 创建一个结构体，实现Future特性
struct DefineFuture;
impl Future for DefineFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _ctx: &mut Context) -> Poll<Self::Output> {
        println!("execute a future. staring...");
        sleep(Duration::from_millis(1500));
        println!("execute a future. ending...");
        Poll::Ready(())
        // unimplemented!()
    }
}
fn fn_future() -> impl Future<Output = ()> {
    DefineFuture
}

fn function_name() {
    println!("I run in core thread.");
}
