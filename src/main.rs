use std::{
    fs,
    io::{prelude::*, BufReader, Error},
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
    
    let request_line = &http_request[0];

    println!("Path is -> {}\nExtension is -> {}", get_path(request_line), mime_type(get_path(request_line)));
    
    read_dir("./expose/");
    println!("Received request line: {}", request_line);


    if request_line == "GET / HTTP/1.1" {
        //let page = fs::read_to_string("hello.html").unwrap();
        let page = generate_html("./expose/");
        
        stream.write_all(ok_builder(page.len(),page).as_bytes()).unwrap();
    } else if request_line == "GET /static/404.css HTTP/1.1" {
        let page = fs::read_to_string("static/404.css").unwrap();
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", mime_type(get_path(request_line)), page.len(), page).as_bytes()).unwrap();
    } else if request_line == "GET /static/200.css HTTP/1.1" {
        let page = fs::read_to_string("static/200.css").unwrap();
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", mime_type(get_path(request_line)), page.len(), page).as_bytes()).unwrap();
    } else if request_line == "GET /test.txt HTTP/1.1" {
        let page = fs::read_to_string("expose/test.txt").unwrap();
        stream.write_all(format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-Length: {}\r\n\r\n{}",
            mime_type(get_path(request_line)), // Determine the MIME type
            get_path(request_line).split('/').last().unwrap_or("file"), // Extract the file name
            page.len(), // File size
            page // File content
        ).as_bytes()).unwrap();
    } else if request_line == "GET /static/1224149.png HTTP/1.1" {
        let page = fs::read("static/1224149.png").unwrap();
        let mime = mime_type(get_path(request_line));
        
        let mut response = Vec::new();
        response.extend_from_slice(format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n", mime, page.len()).as_bytes());
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
        } else if path.ends_with(".txt") {
            "text/plain"
        } else {
            "application/octet-stream"
        };
    
    mime.to_string()
}

fn get_path(request: &str) -> String {
    let spl: Vec<&str> = request.split(" ").collect();
    spl[1].to_string()
}

fn read_dir(path: &str) -> Result<(), Error> {
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(())
}

fn generate_html(directory: &str) -> String {
    let mut html = String::from(
        r#"<!DOCTYPE html><html><head><title>Directory Listing</title>
        <link rel="stylesheet" type="text/css" href="/static/200.css"></head><body>"#
    );
    html.push_str("<h1>Directory Listing</h1><ul>");

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                let link = format!("<li><a href=\"/{0}\">{0}</a></li>", file_name);
                html.push_str(&link);
            }
        }
    } else {
        html.push_str("<p>Error reading directory.</p>");
    }

    html.push_str("</ul></body></html>");
    html
}