use std::env;
use std::fs;
use std::str;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Get arguments
    // let args: Vec<String> = env::args().collect();
    // let root_path = &args[1];
    // println!("Base path: {}", root_path); 

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
    let buffer_in = str::from_utf8(&buffer).unwrap();

    // Print request
    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]));

    // Read first header line
    let mut iter = buffer_in.split_whitespace();
    let req_method = iter.next();
    let path = iter.next();
    // let http_version = iter.next();

    // Get response file based on request
    let (status, filename) = match (req_method, path) {
        (None, _) | (_, None) => ("400 Bad Request", "400.html"),

        // GET Request root page: index.html
        (Some("GET"), Some("/")) => ("200 OK", "index.html"), 

        // GET Request
        (Some("GET"), _) => { 
            match (path) {
                Some(path_str) => {
                    let path_slice = &path_str[1..];
                    if std::path::Path::new(path_slice).exists() {
                        ("200 OK", path_slice)
                    }
                    else { ("404 NOT FOUND", "404.html") }
                },
                None =>  ("404 NOT FOUND", "404.html")
            }
        },

        // POST Request
        (Some("POST"), _) => { 
            ("501 Not Implemented", "501.html")
        },

        // Unsupported request
        _ => ("501 Not Implemented", "501.html"),
    };

    println!("{:?}", (status, filename));

    // Write response
    // TODO fix so that can GET .jpeg files. Currently read_to_string cannot read non-UTF-8
    let contents = fs::read(filename).unwrap();
    let response_header = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n",
        status,
        contents.len(),
    );
    stream.write(response_header.as_bytes()).unwrap();
    stream.write(&contents[..]).unwrap();
    stream.flush().unwrap();
}
