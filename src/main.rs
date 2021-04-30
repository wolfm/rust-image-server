use std::env;

use hyper::{Server, Request, Response};

fn main() {
    // Get arguments
    // let args: Vec<String> = env::args().collect();
    // let root_path = &args[1];
    // println!("Base path: {}", root_path); 

    // Start server
    let server = Server::http("0.0.0.0:0").unwrap().handle(handle_request).unwrap();
}

fn handle_request(hyper_request: Request<Body>, res: Response) {
    let (_, _, headers, _, _, mut reader) = hyper_request.deconstruct();

    let form_data = formdata::read_formdata(&mut reader, &headers).unwrap();

    for (name, value) in form_data.fields {
        println!("Posted field name={}, value={}", name, value);
    }

    for (name, file) in form_data.files {
        println!("Posted file name={}, path = {:?}", name, file.path);
    }

    /*

    // Print request
    // println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]));

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
    */
}
