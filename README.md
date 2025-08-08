# RPL-3.0-demo

Resoluciones

## Cifrado Cesar

Crear caesar_cipher.rs

```rust
pub struct CaesarCipher {
   shift: i32,
}


impl CaesarCipher {
   pub fn new(shift: i32) -> Self {
       CaesarCipher {
           shift: shift,
       }
   }


   pub fn encrypt(&self, message: &str) -> String {
       message.chars().map(|c| {
           if c.is_ascii_alphabetic() {
               let base = if c.is_ascii_uppercase() { 'A' as u8 } else { 'a' as u8 };
               let normalized_shift = ((self.shift % 26) + 26) % 26;
               let shifted = (c as u8 - base + normalized_shift as u8) % 26;
               (shifted + base) as char
           } else {
               c
           }
       }).collect()
   }
}

```

Ejecutrar pruebas luego de crear este archivo

## Contadores compartidos

Reemplazar todo lib.rs por:

```rust
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

```

## Semáforo

Reemplazar todo lib.rs por:

```rust

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

```
