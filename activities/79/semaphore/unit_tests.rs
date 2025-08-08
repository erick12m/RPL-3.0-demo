use student_package::*; // No borrar
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_semaphore_creation() {
    let semaphore = Semaphore::new(5);
    assert_eq!(semaphore.get_value(), 5, "El valor inicial del semáforo no es el esperado");
}

#[test]
fn test_semaphore_acquire() {
    let semaphore = Semaphore::new(3);
    semaphore.acquire();
    assert_eq!(semaphore.get_value(), 2, "El valor del semáforo después de acquire no es el esperado");
}

#[test]
fn test_semaphore_release() {
    let semaphore = Semaphore::new(1);
    semaphore.release();
    assert_eq!(semaphore.get_value(), 2, "El valor del semáforo después de release no es el esperado");
}

#[test]
fn test_semaphore_acquire_release_cycle() {
    let semaphore = Semaphore::new(2);
    
    // Adquirir dos veces
    semaphore.acquire();
    semaphore.acquire();
    assert_eq!(semaphore.get_value(), 0, "El semáforo debería estar en 0 después de dos acquires");
    
    // Liberar una vez
    semaphore.release();
    assert_eq!(semaphore.get_value(), 1, "El semáforo debería estar en 1 después del release");
    
    // Adquirir una vez más
    semaphore.acquire();
    assert_eq!(semaphore.get_value(), 0, "El semáforo debería estar en 0 después del acquire");
}

#[test]
fn test_concurrent_acquire_release() {
    let semaphore = Arc::new(Semaphore::new(1));
    let num_threads = 5;
    let mut handles = Vec::new();
    
    // Crear hilos que adquieren y liberan el semáforo
    for i in 0..num_threads {
        let semaphore_clone = semaphore.clone();
        let handle = thread::spawn(move || {
            semaphore_clone.acquire();
            // Simular trabajo
            thread::sleep(Duration::from_millis(10));
            semaphore_clone.release();
        });
        handles.push(handle);
    }
    
    // Esperar a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }
    
    // El valor final debería ser 1 (el valor inicial)
    assert_eq!(semaphore.get_value(), 1, "El valor final del semáforo no es el esperado");
}

#[test]
fn test_semaphore_with_multiple_permits() {
    let semaphore = Arc::new(Semaphore::new(3));
    let num_threads = 6;
    let mut handles = Vec::new();
    let counter = Arc::new(std::sync::Mutex::new(0));
    
    // Crear hilos que adquieren el semáforo
    for _ in 0..num_threads {
        let semaphore_clone = semaphore.clone();
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            semaphore_clone.acquire();
            // Incrementar contador para verificar concurrencia
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
            // Simular trabajo
            thread::sleep(Duration::from_millis(50));
            *count -= 1;
            semaphore_clone.release();
        });
        handles.push(handle);
    }
    
    // Esperar un poco para que algunos hilos adquieran el semáforo
    thread::sleep(Duration::from_millis(25));
    
    // Verificar que máximo 3 hilos pueden estar ejecutándose simultáneamente
    let current_count = *counter.lock().unwrap();
    assert!(current_count <= 3, "Más de 3 hilos están ejecutándose simultáneamente");
    
    // Esperar a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(semaphore.get_value(), 3, "El valor final del semáforo no es el esperado");
}