use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of thread in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` functoin will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some thread and store them in the vector
        }
        ThreadPool { threads }
    }

    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {

    // }

    pub fn execute<F>(&self, f: F) where 
        F: FnOnce() + Send + 'static,
    {

    }
}