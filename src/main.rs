use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        //println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    for iter in &http_request {
        println!("----> {iter}");
    }
    
    let request_line = &http_request[0];

    println!("Path is -> {}\nExtension is -> {}", get_path(request_line), mime_type(get_path(request_line)));


    if (request_line == "GET / HTTP/1.1") {
        let page = fs::read_to_string("hello.html").unwrap();
        stream.write_all(ok_builder(page.len(),page).as_bytes()).unwrap();
    } else if (request_line == "GET /static/404.css HTTP/1.1") {
        let page = fs::read_to_string("static/404.css").unwrap();
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", mime_type(get_path(request_line)), page.len(), page).as_bytes()).unwrap();
    } else if (request_line == "GET /static/1224149.png HTTP/1.1") {
        let page = fs::read("static/1224149.png").unwrap();
        let mime = mime_type(get_path(request_line));
        
        let mut response = Vec::new();
        response.extend_from_slice(format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                mime, page.len()
        ).as_bytes());
        response.extend_from_slice(&page);

        stream.write_all(&response).unwrap();

    } else {
        let page = fs::read_to_string("404.html").unwrap();
        stream.write_all(format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", page.len(), page).as_bytes()).unwrap();
    }
}

fn ok_builder(length: usize, contents: String) -> String {
    return format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");
}

fn nok_builder(length: usize, contents: String) -> String {
    return format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {length}\r\n\r\n{contents}");
}

fn mime_type(path: String) -> String {
    let contents = fs::read(path.clone());
    let mime = if path.ends_with(".css") {
            "text/css"
        } else if path.ends_with(".png") {
            "image/png"
        } else {
            "application/octet-stream"
        };
    
    mime.to_string()
}

fn get_path(request: &str) -> String {
    let spl: Vec<&str> = request.split(" ").collect();
    spl[1].to_string()
}
