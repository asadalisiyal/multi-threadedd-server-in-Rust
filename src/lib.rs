use std::{thread, sync::{mpsc::{self, Receiver}, Arc, Mutex}}; 

pub struct ThreadPool {
    workers: Vec<Worker>, 
    sender: mpsc::Sender<Job>,
}

pub struct Job;

impl ThreadPool {

    // create a new thread pool
    // size is number of threads and it should be greater then zero

    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);

        let (sender , receiver ) = mpsc::channel(); 
        let receiver = Arc::new(Mutex::new(receiver)); 

        let mut workers = Vec::with_capacity(size); 

        for id in 0..size {
            // create threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));


        }
        ThreadPool {workers, sender}
    }


    pub fn execute <F> (&self, f: F) 
    where 
    F: FnOnce() + Send + 'static
    {
        let job = Box::new(f); 
        self.sender.send(Job).unwrap(); 
    }
}

struct Worker {
    id: usize, 
    thread: thread::JoinHandle<()>
}
impl Worker {

    fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn( move || loop {
            let job = receiver
            .lock()
            .unwrap()
            .recv()
            .unwrap(); 

            println!("Worker {} got a job ; Executing ", id); 
            // receiver
            Job;

        }); 
        Worker { id , thread }
    }
    
}