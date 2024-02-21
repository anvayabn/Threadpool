/*
    Author: Anvaya B Narappa
    emailid: an001@ucr.edu

    The Goal of this program is to implement a Thread Pool
    A thread pool which can contain a number of predetermined threads
    everytime, new threads can be spawned from the thread pool and computations can be run on it

 */
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Threadpool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Option<Job>>,
}

impl Threadpool {
    fn new(n: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Option<Job>>();
        let arx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(n);

        for id in 0..n{
            let arx_clone = Arc::clone(&arx);
            let handle = thread::spawn( move || loop {
                let job = arx_clone.lock().unwrap().recv();
                match job {
                    Ok(Some(job)) => {
                        job();
                        println!("Thread {id} completed !!")
                    },
                    Ok(None) => {break;},
                    Err(_) => {
                        break;
                    }
                }
            });

            workers.push(handle);
        }

        Threadpool {workers, sender:tx}

    }

   fn send_job<F>(&self, job: F)
        where
            F: FnOnce() + Send + 'static,
   {
       self.sender.send(Some(Box::new(job))).unwrap();
   }

    fn complete_and_return(self){
        for _ in &self.workers{
            self.sender.send(None).unwrap();
        }

        for handle in self.workers{
            handle.join().unwrap();
        }
    }
}

fn dummy_threadfn(){
    let mut count = 0 ;
    for _ in 0..100 {
        count += 1 ;
    }
}


fn main() {


    //initialise a 3 thread, Threadpool
    let tpool = Threadpool::new(3);

    for _ in 0..10 {
        tpool.send_job(dummy_threadfn);
    }

    tpool.complete_and_return();
}
