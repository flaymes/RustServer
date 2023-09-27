use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use RustServer::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7070").unwrap();
    let pool = ThreadPool::new(4);

     for connection in listener.incoming().take(2) {
        let stream = connection.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_method_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content_file) = match &req_method_line[..] {
        "GET / HTTP/1.1" => { ("HTTP/1.1 200 OK", "hello.html") }
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => { ("HTTP/1.1 404 NOT FOUND", "404.html") }
    };

    let contents = fs::read_to_string(content_file).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).expect("写入结果失败");
}
