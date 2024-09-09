use std::thread::{self, JoinHandle};



pub struct ThreadPool{
    workers: Vec<Worker>
}

impl ThreadPool
{
    pub fn new(size: usize) -> ThreadPool
    {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);
        for id in 0..size
        {
            threads.push(Worker::new(id));
        }
        ThreadPool {workers: threads}
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        
    }
}

struct Worker 
{
    id: usize,
    job: JoinHandle<()>
}

impl Worker
{
    fn new(id: usize) -> Worker
    {
        let worker = Worker {id: id, job: thread::spawn(|| {})};
        return worker
    }
}