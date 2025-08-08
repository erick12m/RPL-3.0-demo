use std::sync::{Arc, Mutex, Condvar};

pub struct Semaphore {
    inner: Arc<(Mutex<i32>, Condvar)>,
}

impl Semaphore {
    pub fn new(initial_value: i32) -> Self {
        
    }

    pub fn acquire(&self) {
        
    }

    pub fn release(&self) {
        
    }

    pub fn get_value(&self) -> i32 {
        
    }
} 