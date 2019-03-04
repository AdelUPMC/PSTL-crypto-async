extern crate rand;
extern crate time;
//extern crate crypto_async;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use rand::prelude::*;


use time::PreciseTime;


fn main() {
	let start = PreciseTime::now();
    let mut vec_thread = Vec::new();
    //Send 100 requests at the same time
    for _i in 0..64 {
        let handle = thread::spawn(move || {
            let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

            //write
            let rand_u64: u64 = rand::thread_rng().gen();

            let request: String = rand_u64.to_string();
            stream.write(request.as_bytes()).unwrap();

            //read
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
			//crypto_async::wrapper_des::des(plain, key, &mut cipher);
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