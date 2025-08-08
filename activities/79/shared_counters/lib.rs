use std::sync::{Arc, Mutex};

pub struct SharedCounter {
    value: Arc<Mutex<i32>>,
}

impl SharedCounter {
    pub fn new(value: i32) -> Self {
        SharedCounter { value: Arc::new(Mutex::new(value)) }
    }

    pub fn get_value(&self) -> i32 {
        let value = self.value.lock().unwrap();
        *value
    }

    pub fn increment(&self) {
        let mut value = self.value.lock().unwrap();
        *value += 1;
    }

    pub fn sum(&self, n: i32) { 
        let mut value = self.value.lock().unwrap();
        *value += n;
    }       
}