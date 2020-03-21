use std::thread;
use std::sync::{mpsc, Mutex, Arc};

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>
}

type Job = Box<FnOnce() + Send + 'static>;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>
}

type ThreadSafeReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;

impl Worker {
  fn new(id: usize, receiver: ThreadSafeReceiver) -> Worker {
    let thread = thread::spawn(|| { receiver; });
    Worker {
      id,
      thread
    }
  }
}

impl ThreadPool {

  /// Create new ThreadPool
  ///
  /// Size is the number of threads in the thread pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool{
    assert!(size > 0);
    let mut workers = Vec::with_capacity(size);
    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));
    for id in 0..size {
      workers.push(Worker::new(id, receiver.clone()))
    };
    ThreadPool {
      workers,
      sender
    }
  }

  /// Executes a function on a thread pool.
  pub fn execute<F>(&self, f: F) where F : FnOnce() + Send + 'static {
    let job = Box::new(f);
    self.sender.send(job);
  }
}