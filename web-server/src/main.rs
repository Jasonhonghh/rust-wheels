use std::{
    fs,
    thread,
    time::Duration,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use web_server::ThreadPool;
// use log;
// use tracing::subscriber::registry;

fn main() {
    let listener = TcpListener::bind("218.199.84.181:7876").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        println!("Connection established");
        pool.execute({||tcp_handle(stream.unwrap())});
    }
    println!("Shutting down.")
}

pub fn tcp_handle(mut stream:TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let (status_line,file_name) = match request_line.as_str(){
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK","hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK","hello.html")},
        _ => ("HTTP/1.1 404 NOT FOUND","404.html"),
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);
    stream.write_all(response.as_bytes()).unwrap();
    println!("Connection established");
}