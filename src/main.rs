use std::env;
use std::fs;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    let root_path = &args[1];
    println!("Base path: {}", root_path); 

    // Setup TCP Listener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_header = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get_header) {
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        println!("Unsupported Request");
    }

}
