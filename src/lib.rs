use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}
pub struct ThreadPool {
    workers: Vec<Worker>,
}
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
	/// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
	
	 pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
		let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		 for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
         }
    
        ThreadPool {
			workers,
            sender,
        }
    }

	 pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
	 {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
     }
    
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

				let output = scan!("2 false fox", char::is_whitespace, u8, bool, String);
				println!("{:?}", output); // (Some(2), Some(false), Some("fox"))
                //println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}
