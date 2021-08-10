use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ThreadPool {
    running: bool,
    workers: usize,
    threads: Vec<thread::Thread>,
    //jobs: Mutex<Vec<Box<&'static (dyn Fn() + 'static)>>>,
    jobs: Mutex<Vec<Box<dyn FnOnce() + Send + Sync + 'static>>>,
}

impl ThreadPool {
    fn new(num_workers: usize) -> Self {
        Self {
            running: false,
            workers: num_workers,
            jobs: Mutex::new(Vec::new()),
            threads: Vec::new(),
        }
    }

    pub fn add_job<T>(&mut self, job: T) where T: FnMut() + Send + Sync + 'static {
        self.jobs.lock().unwrap().push(Box::new(job));
    }

    pub fn start(&mut self) -> Result<(), &str> {
        //thread::spawn(|| {
            while self.running {
                let job = self.jobs.lock().unwrap().pop().unwrap();

                if self.threads.len() < self.workers {
                    if self.jobs.lock().unwrap().len() > 0 {
                        self.run_next_job();
                    } else {
                        thread::sleep(Duration::from_millis(200));
                    }
                }
            }
        //});
        Ok(())
    }

    pub fn run_next_job(&mut self) -> Result<(), &str> {
        let job = self.jobs.lock().unwrap().pop().unwrap();

        if self.threads.len() < self.workers {
            thread::spawn(move || {
                job();
            });
            Ok(())
        } else {
            Err("no available worker")
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn complete(&mut self) {
        self.stop();
        self.run_next_job();
        thread::sleep(std::time::Duration::from_millis(1000));
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
    pub fn jobs_remaining(&self) -> usize {
        self.jobs.lock().unwrap().len()
    }

}

#[cfg(test)]
mod tests {
    use crate::thread_pool::*;

    #[test]
    fn test() {
        let mut threadpool = ThreadPool::new(2);

            println!("hello");
        threadpool.add_job(|| {
            println!("there");
        });
        //threadpool.start();
        threadpool.complete();
        assert!(false);
    }
}
