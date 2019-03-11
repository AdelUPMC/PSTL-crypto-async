extern crate rand;
extern crate time;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::env;
use rand::prelude::*;



use time::PreciseTime;


fn main() {
	let args: Vec<String> = env::args().collect();
	let nbrequest: usize = args[1].parse().unwrap();
	let start = PreciseTime::now();
    let mut vec_thread = Vec::new();
    
    for _i in 0..nbrequest {
        let handle = thread::spawn(move || {
            let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

            //write
            let rand_u64: u64 = rand::thread_rng().gen();

            let request: String = rand_u64.to_string();
            stream.write(request.as_bytes()).unwrap();

            //read
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            println!("Received : {}", String::from_utf8_lossy(&buffer));
        });
        vec_thread.push(handle);
    }

    for t in vec_thread {
        t.join().unwrap();
    }
	let end = PreciseTime::now();
	println!("{} seconds for executing several client threads", start.to(end));
}