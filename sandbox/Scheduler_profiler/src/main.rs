use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use web_server::{ThreadPool, Cell};
use std::sync::{Arc, Mutex, mpsc};
use rand::prelude::*;
use std::env;
use std::{thread, time};


fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let vec: Vec<Cell> = vec![Cell { plain: 0, key: 0 }; size];
    let vec: Arc<Mutex<Vec<Cell>>> = Arc::new(Mutex::new(vec));

    let counter = Arc::new(Mutex::new(0));
    let counter_write = Arc::new(Mutex::new(0));
    let pool = ThreadPool::new(size);
    let (sender, receiver) = mpsc::channel();
    let (s, r) = mpsc::channel();
    let r = Arc::new(Mutex::new(r));
    let s = Arc::new(Mutex::new(s.clone()));


    let receiver = Arc::new(Mutex::new(receiver));
    let sender = Arc::new(Mutex::new(sender.clone()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let sender = Arc::clone(&sender);
        let receiver = Arc::clone(&receiver);
        let r = Arc::clone(&r);
        let s = Arc::clone(&s);
        let counter = Arc::clone(&counter);
        let buffer = Arc::clone(&vec);
        let counter_write = Arc::clone(&counter_write);

        pool.execute(move || {
            handle_connection(stream, counter, buffer, sender, receiver, size, counter_write, s, r);
        });
    }
}


pub fn handle_connection(mut stream: TcpStream, counter: Arc<Mutex<i32>>, buf: Arc<Mutex<Vec<Cell>>>,
                         sen: Arc<Mutex<mpsc::Sender<()>>>, rec: Arc<Mutex<mpsc::Receiver<()>>>,
                         size: usize, counter_write: Arc<Mutex<i32>>, s: Arc<Mutex<mpsc::Sender<()>>>,
                         r: Arc<Mutex<mpsc::Receiver<()>>>) {
    let mut buffer = [0; 8];
    stream.read(&mut buffer).unwrap();
    let plain = u64::from_be_bytes(buffer);
    let key = rand::thread_rng().gen();


    let mut cpt = counter.lock().unwrap();
    if *cpt == -1 {
        let _received = r.lock().unwrap().recv().unwrap();
        *cpt = 0;
    }
    let index = *cpt;
    //std::mem::drop(cpt);

    assert!(index >= 0 && index <= size as i32);
    let mut buff = buf.lock().unwrap();

    buff[index as usize].key = key;
    buff[index as usize].plain = plain;
    std::mem::drop(buff);
    //let mut cpt = counter.lock().unwrap();
    *cpt += 1;
    std::mem::drop(cpt);

    if index == (size as i32 - 1) {
        let mut buff = buf.lock().unwrap();
        for i in 0..(size) {
            buff[i].plain ^= buff[i].key;
            thread::sleep(time::Duration::from_millis(1));
        }
        assert!(buff[size - 1].plain == key ^ plain);
        stream.write(&u64_to_array_of_u8(buff[size - 1].plain)).unwrap();
        let mut cpt = counter.lock().unwrap();
        *cpt = -1;
        for _ in 0..(size - 1) {
            let _sender = sen.lock().unwrap().send(()).unwrap();
        }
        std::mem::drop(buff);
    } else {
        let _received = rec.lock().unwrap().recv().unwrap();
        let buff = buf.lock().unwrap();
        let result = buff[index as usize].plain;
        assert!(result == key ^ plain);
        stream.write(&u64_to_array_of_u8(result)).unwrap();
        let mut c = counter_write.lock().unwrap();
        *c += 1;
        if *c == (size as i32) - 1 {
            let _sender = s.lock().unwrap().send(()).unwrap();
            *c = 0;
        }
    }
}


fn u64_to_array_of_u8(x: u64) -> [u8; 8] {
    let b1: u8 = ((x >> 56) & 0xff) as u8;
    let b2: u8 = ((x >> 48) & 0xff) as u8;
    let b3: u8 = ((x >> 40) & 0xff) as u8;
    let b4: u8 = ((x >> 32) & 0xff) as u8;
    let b5: u8 = ((x >> 24) & 0xff) as u8;
    let b6: u8 = ((x >> 16) & 0xff) as u8;
    let b7: u8 = ((x >> 8) & 0xff) as u8;
    let b8: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4, b5, b6, b7, b8];
}