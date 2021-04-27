use std::env;
use std::fs;
use std::str;

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
        println!("Connection received");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let buffer_in = str::from_utf8(&buffer).unwrap();

    // Print request
    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]));

    // Read first header line
    let mut iter = buffer_in.split_whitespace();
    let req_method = iter.next();
    let path = iter.next();
    let http_version = iter.next();

    let (status, filename) = match (req_method, path) {
        (None, _) | (_, None) => ("400 Bad Request", "400.html"),
        (Some("GET"), Some("/index.html")) => ("200 OK", "index.html"), 
        (Some("GET"), _) => ("404 NOT FOUND", "404.html"),
        _ => ("501 Not Implemented", "501.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    // println!("{}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
