// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    println!("accepted new connection");

    let mut request_data = String::new();

    let mut request_buffer = [0; 1024];

    match stream.read(&mut request_buffer) {
        Ok(read_bytes) => {
            println!("read_bytes {read_bytes}");
            request_data.extend(String::from_utf8_lossy(&request_buffer[0..read_bytes]).chars());
        }
        Err(err) => {
            eprintln!("failed to read request {err}");
        }
    }

    println!("request-data {request_data}");

    let mut lines = request_data.lines();

    let startline = lines.next().expect("Missing start line");

    println!("startline -->|{startline}");

    let mut segments = startline.split_whitespace();
    let _ = segments.next().unwrap();
    let path = segments.next().unwrap();

    println!("path -> {path}");

    match path {
        "/" => {
            let _ = stream
                .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                .map_err(|err| eprintln!("failed to write to stream {err}"));
        }
        _ => {
            if path.starts_with("/echo/") {
                let random_string = path.strip_prefix("/echo/").expect("failed to strip /echo/");

                println!("random-string {random_string}");

                let content_length = random_string.len();

                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {content_length}\r\n\r\n{random_string}\r\n");

                let _ = stream
                    .write(response.as_bytes())
                    .map_err(|err| eprintln!("failed to write to stream {err}"));
            } else if path == "/user-agent" {
                let _ = lines.next();
                let user_agent_line = lines.next().expect("missing user agent line");
                let user_agent = user_agent_line
                    .split(": ")
                    .skip(1)
                    .next()
                    .expect("missing user-agent");

                let content_length = user_agent.len();

                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {content_length}\r\n\r\n{user_agent}\r\n");

                let _ = stream
                    .write(response.as_bytes())
                    .map_err(|err| eprintln!("failed to write to stream {err}"));
            } else {
                let _ = stream
                    .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                    .map_err(|err| eprintln!("failed to write to stream {err}"));
            }
        }
    }

    let _ = stream
        .shutdown(std::net::Shutdown::Both)
        .map_err(|err| eprintln!("failed to shutdown connection {err}"));
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _handle = thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
