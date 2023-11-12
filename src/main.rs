// Uncomment this block to pass the first stage
use std::io::Write;
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let _ = stream
                    .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                    .map_err(|err| eprintln!("failed to write to stream {err}"));
                let _ = stream
                    .shutdown(std::net::Shutdown::Both)
                    .map_err(|err| eprintln!("failed to shutdown connection {err}"));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
