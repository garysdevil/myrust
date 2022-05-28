// 异步运行时 tokio

#![allow(dead_code)]

use std::future::Future;
use std::thread::sleep;
use std::time::Duration;

use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::main]
#[cfg(feature = "feature1")]
pub async fn main() {
    // This is running on a core thread.
    println!("We are in pratice_2.rs");

    fn_for();

    fn_future().await;

    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a blocking thread.
        // Blocking here is ok.
        println!("I am blocking_task");
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
    blocking_task.await.unwrap();
}

// 创建一个结构体，实现Future特性
struct DoNothing;
impl Future for DoNothing {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context) -> Poll<Self::Output> {
        println!("DoNothing {}", "--");
        Poll::Ready(())
        // unimplemented!()
    }
}

fn fn_future() -> impl Future<Output = ()> {
    DoNothing
}

fn fn_for() {
    for i in 1..=10 {
        println!("sleep_fn {}", i);
        sleep(Duration::from_millis(500));
    }
}
