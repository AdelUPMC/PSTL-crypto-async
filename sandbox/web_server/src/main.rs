use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use web_server::ThreadPool;
use std::io::Read;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
		match stream {
            Ok(stream) => {
                pool.execute(|| {
            handle_connection(stream);
			});
            }
            Err(_) => {
                prinln!("Error");
            }
        }

    }
}


fn handle_connection(mut stream: TcpStream) {
	loop {
        let mut buffer = [0;512];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 { 
                    // connection was closed
                    break;
                }
                stream.write(&buffer[0..n]).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}