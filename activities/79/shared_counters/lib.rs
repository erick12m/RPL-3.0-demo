use std::sync::{Arc, Mutex};

pub struct SharedCounter {
    value: Arc<Mutex<i32>>,
}

impl SharedCounter {
    pub fn new(value: i32) -> Self {
        
    }

    pub fn get_value(&self) -> i32 {
        
    }

    pub fn increment(&self) {
        
    }

    pub fn sum(&self, n: i32) { 
        
    }       
}
