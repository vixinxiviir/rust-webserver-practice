use std::thread::{self, JoinHandle};
use std::sync::{mpsc, Arc, Mutex};


type Job = Box<dyn FnOnce() + Send + 'static>;
type MTReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        drop(self.sender.take());
        for worker in &mut self.workers
        {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.job.take()
            {
                thread.join().unwrap();
            }
            
        }
    }
}
impl ThreadPool
{
    pub fn new(size: usize) -> ThreadPool
    {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);

        for id in 0..size
        {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {workers: threads, sender: Some(sender)}
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker 
{
    id: usize,
    job: Option<JoinHandle<()>>
}

impl Worker
{
    fn new(id: usize, receiver: MTReceiver) -> Worker
    {
        let thread = thread::spawn(move || loop {
            {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => 
                    {
                        println!("worker {id} got a job; executing...");
                        job();
                    }
                    Err(_) =>
                    {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
                
            }
        });
        Worker {id: id, job: Some(thread)}
    }
}

