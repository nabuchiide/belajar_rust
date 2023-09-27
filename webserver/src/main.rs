use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8089").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
    println!("Request: {:#?}", http_request);
    stream.write_all(response.as_bytes()).unwrap();
}
