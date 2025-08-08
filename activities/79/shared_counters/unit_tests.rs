use student_package::*; // No borrar
use std::sync::Arc;
use std::thread;

#[test]
fn test_shared_counter_creation() {
    let counter = SharedCounter::new(5);
    assert_eq!(counter.get_value(), 5, "El valor del contador no es el esperado");
}

#[test]
fn test_shared_counter_increment() {
    let counter = SharedCounter::new(5);
    counter.increment();
    assert_eq!(counter.get_value(), 6, "El valor del contador no es el esperado");
}

#[test]
fn test_shared_counter_sum() {
    let counter = SharedCounter::new(0);
    counter.sum(5);
    assert_eq!(counter.get_value(), 5, "El valor del contador no es el esperado");
}

#[test]
fn test_concurrent_increments() {
    let counter = Arc::new(SharedCounter::new(0));
    let num_threads = 3;
    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(counter.get_value(), num_threads * 100, "El valor del contador no es el esperado");
}

#[test]
fn test_concurrent_sum() {
    let counter = Arc::new(SharedCounter::new(0));
    let num_threads = 3;
    let mut handles = Vec::new();
    
    for _ in 0..num_threads {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            for i in 0..10 {
                counter_clone.sum(i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(counter.get_value(), 45 * num_threads, "El valor del contador no es el esperado");
}
