use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_request(&mut stream);
    }
}


fn handle_request(stream : &mut TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]) );

    let resp = "http/1.1 200 OK\r\n\r\n";
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
