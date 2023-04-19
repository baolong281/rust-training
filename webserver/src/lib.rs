use std::{
	thread::{self}, 
	sync::{mpsc, Arc, Mutex},
};

pub struct ThreadPool {
	workers: Vec<Worker>,	
	sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(move || loop {
			let message = receiver.lock().unwrap().recv();

			match message { 
				Ok(job) => {
					println!("Worker {id} executing");
					job();
				}
				Err(_) => {
					println!("Worker {id} shutting down");
					break;
				}
			}

		});

		Worker {
			id,
			thread: Some(thread)
		}

	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		drop(self.sender.take());

		for worker in &mut self.workers {
			println!("Shutting worker {}", worker.id);
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap()
			}
		}
	}
}

impl ThreadPool {
	pub fn build(size: usize) -> Result<ThreadPool, & 'static str> {
		let mut workers= Vec::with_capacity(size);
		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		for i in 0..size {
			workers.push(Worker::new(i, Arc::clone(&receiver)));
		}

		return Ok(ThreadPool { workers, sender: Some(sender)});
	}

	pub fn execute<F>(&self, f: F)
	where
		F: FnOnce() + Send + 'static,
		{
			let job = Box::new(f);
			self.sender.as_ref().unwrap().send(job).unwrap();
		}
}