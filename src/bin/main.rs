use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        pool.execute(move || handle_request(&mut stream));
    }

    // for stream in listener.incoming() {
    //     let mut stream = stream.unwrap();
    //     handle_request(&mut stream);
    // }
}

fn handle_request(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let get_css = b"GET /style.css HTTP/1.1";
    let _get_favicon = b"GET /favicon.ico HTTP/1.1";

    let (status_line, filename) = if buffer.starts_with(get) {
        thread::sleep(Duration::from_secs(3));
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(get_css) {
        ("HTTP/1.1 200 OK", "style.css")
    } else {
        ("HTTP/1.1 200 OK", "404.html")
    };

    let html_page = fs::read_to_string(filename).unwrap();

    let resp = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html_page.len(),
        html_page
    );
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
