use student_package::*; // No borrar
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_semaphore_creation() {
    let semaphore = Semaphore::new(5);
    assert_eq!(semaphore.get_value(), 5, "El valor inicial del semaforo no es el esperado");
}

#[test]
fn test_semaphore_acquire() {
    let semaphore = Semaphore::new(3);
    semaphore.acquire();
    assert_eq!(semaphore.get_value(), 2, "El valor del semaforo despues de acquire no es el esperado");
}

#[test]
fn test_semaphore_release() {
    let semaphore = Semaphore::new(1);
    semaphore.release();
    assert_eq!(semaphore.get_value(), 2, "El valor del semaforo despues de release no es el esperado");
}

#[test]
fn test_semaphore_acquire_release_cycle() {
    let semaphore = Semaphore::new(2);
    
    // Adquirir dos veces
    semaphore.acquire();
    semaphore.acquire();
    assert_eq!(semaphore.get_value(), 0, "El semaforo deberia estar en 0 despues de dos acquires");
    
    // Liberar una vez
    semaphore.release();
    assert_eq!(semaphore.get_value(), 1, "El semaforo deberia estar en 1 despues del release");
    
    // Adquirir una vez mas
    semaphore.acquire();
    assert_eq!(semaphore.get_value(), 0, "El semaforo deberia estar en 0 despues del acquire");
}

#[test]
fn test_concurrent_acquire_release() {
    let semaphore = Arc::new(Semaphore::new(1));
    let num_threads = 5;
    let mut handles = Vec::new();
    
    // Crear hilos que adquieren y liberan el semaforo
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
    
    // El valor final deberia ser 1 (el valor inicial)
    assert_eq!(semaphore.get_value(), 1, "El valor final del semaforo no es el esperado");
}

#[test]
fn test_semaphore_with_multiple_permits() {
    let semaphore = Arc::new(Semaphore::new(3));
    let num_threads = 6;
    let mut handles = Vec::new();
    let counter = Arc::new(std::sync::Mutex::new(0));
    
    // Crear hilos que adquieren el semaforo
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
    
    // Esperar un poco para que algunos hilos adquieran el semaforo
    thread::sleep(Duration::from_millis(25));
    
    // Verificar que maximo 3 hilos pueden estar ejecutandose simultaneamente
    let current_count = *counter.lock().unwrap();
    assert!(current_count <= 3, "Mas de 3 hilos estan ejecutandose simultaneamente");
    
    // Esperar a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(semaphore.get_value(), 3, "El valor final del semaforo no es el esperado");
}