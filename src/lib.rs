use std::sync::mpsc;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

// workaround for rust issue
trait FnBox {
    fn call_box(self: Box<Self>);
}
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the number of threads is zero.
    pub fn new(num_threads: usize) -> Self {
        assert!(num_threads > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&mut self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker { id, thread }
    }
}
