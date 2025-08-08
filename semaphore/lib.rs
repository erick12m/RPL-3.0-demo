use std::sync::{Arc, Mutex, Condvar};

pub struct Semaphore {
    inner: Arc<(Mutex<i32>, Condvar)>,
}

impl Semaphore {
    pub fn new(initial_value: i32) -> Self {
        if initial_value < 0 {
            panic!("El valor inicial del semáforo debe ser no negativo");
        }
        Semaphore {
            inner: Arc::new((Mutex::new(initial_value), Condvar::new())),
        }
    }

    pub fn acquire(&self) {
        let (lock, cvar) = &*self.inner;
        let mut value = lock.lock().unwrap();
        
        while *value <= 0 {
            value = cvar.wait(value).unwrap();
        }
        
        *value -= 1;
    }

    pub fn release(&self) {
        let (lock, cvar) = &*self.inner;
        let mut value = lock.lock().unwrap();
        
        *value += 1;
        cvar.notify_one();
    }

    pub fn get_value(&self) -> i32 {
        let (lock, _) = &*self.inner;
        let value = lock.lock().unwrap();
        *value
    }
} 