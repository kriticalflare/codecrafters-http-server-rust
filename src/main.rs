// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{fs, thread};

use nom::AsBytes;

fn handle_client(mut stream: TcpStream, file_directory_path: Option<String>) {
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
            } else if path.starts_with("/files/") {
                let file_name = path
                    .strip_prefix("/files/")
                    .expect("failed to strip /files/");

                match file_directory_path {
                    Some(dir_path) => {
                        let file_path = Path::new(&dir_path).join(file_name);
                        match fs::read(file_path) {
                            Ok(file) => {
                                let content_length = file.len();
                                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {content_length}\r\n\r\n");

                                let _ = stream
                                    .write(response.as_bytes())
                                    .map_err(|err| eprintln!("failed to write to stream {err}"));

                                let _ = stream.write_all(&file).map_err(|err| {
                                    eprintln!("failed to write file to stream {err}")
                                });
                            }
                            Err(_) => {
                                let _ = stream
                                    .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                                    .map_err(|err| eprintln!("failed to write to stream {err}"));
                            }
                        }
                    }
                    None => {
                        let _ = stream
                            .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                            .map_err(|err| eprintln!("failed to write to stream {err}"));
                    }
                }
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

    let mut file_directory: Option<String> = None;
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        3 => {
            let command = &args[1];
            if command == "--directory" {
                file_directory = Some(args[2].clone());
            }
        }
        _ => {}
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let file_directory = file_directory.clone();
                let _handle = thread::spawn(move || {
                    handle_client(stream, file_directory);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
