use myrust::pratice::thread_pool::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let pool = ThreadPool::new(2);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("I am listening 127.0.0.1:7878");

    for stream in listener.incoming() {
        // for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("WebServer is shutting down.");
}

// 处理发送过来的请求
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let path_get = b"GET /get HTTP/1.1\r\n";
    let path_sleep = b"GET /sleep HTTP/1.1\r\n";

    // if let Ok(str) = std::str::from_utf8(&buffer){
    //     println!("{:?}", &str[0..10]);
    // }
    println!("{:?}", &std::str::from_utf8(&buffer).unwrap()[0..10]);

    let (status_line, filename) = if buffer.starts_with(path_get) {
        ("HTTP/1.1 200 OK", "src/pratice/web_hello.html")
    } else if buffer.starts_with(path_sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "src/pratice/web_hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/pratice/web_404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
