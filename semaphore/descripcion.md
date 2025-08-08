## Implementación de Semáforo

Implementa un semáforo en Rust que permita controlar el acceso concurrente a recursos compartidos. 

![Semáforo](https://media.lmneuquen.com/p/6807e816907a617a5d882269918eb909/adjuntos/195/imagenes/007/773/0007773105/770x0/smart/untitled-designpng.png)

 Un semáforo es una estructura de datos que mantiene un contador que puede ser incrementado o decrementado de manera atómica.

### Funcionalidades requeridas:

1. **Constructor**: Crear un semáforo con un valor inicial específico
2. **Acquire (wait)**: Decrementar el contador del semáforo. Si el contador es 0, el hilo debe esperar hasta que otro hilo libere el semáforo
3. **Release (signal)**: Incrementar el contador del semáforo y despertar a un hilo en espera si existe
4. **Get value**: Obtener el valor actual del contador del semáforo

