use std::thread;
use std::sync::{Arc};
use std::time::Duration;
use std::sync::RwLock;

struct Writer {
    name: String,
}

impl Writer {

    fn new(name: &str) -> Writer {
        Writer {
            name: name.to_string(),
        }
    }

    fn write(&self, rw_lock: &RwLock<i32>) {
        loop {
            // write(): acesso exclusivo
            if let Ok(mut write_guard) = rw_lock.write() {
                // o read_guard retornado também implementa 'Deref'
                *write_guard += 1;
                println!("{} UPDATED value to {}", self.name, *write_guard);
            }
            // quando o guard sai do escopo, escritor perde o lock
            thread::sleep(Duration::from_millis(1000));
        }
    }

}

struct Reader {
    name: String,
}

impl Reader {

    fn new(name: &str) -> Reader {
        Reader {
            name: name.to_string(),
        }
    }

    fn read(&self, rw_lock: &RwLock<i32>) {
        loop {
            // read() bloqueia se um escritor possui o write() lock
            if let Ok(read_guard) = rw_lock.read() {
                // o read_guard retornado também implementa 'Deref'
                println!("{} READ value {}", self.name, *read_guard);
            }
            // quando o guard sai do escopo, leitor perde o lock
            thread::sleep(Duration::from_millis(500));
        }
    }

}

fn main() {
    // cria um i32 com valor inicial de 0
    let value:i32 = 0;
    // ARC: atomic reference count: para compartilhar o rwLock entre múltiplas threads
    // RwLock: permite um determinado número de leitores ou no máximo 1 escritor num determinado momento
    let rw_lock = Arc::new(RwLock::new(value));

    // criação dos leitores
    let readers = vec![
        Reader::new("Judith Butler"),
        Reader::new("Gilles Deleuze"),
        Reader::new("Karl Marx"),
        Reader::new("John Locke"),
        Reader::new("Michel Foucault"),
    ];

    // criação dos escritores
    let writers = vec![
        Writer::new("Socrates"),
        Writer::new("Plato"),
        Writer::new("Aristotle"),
    ];

    // handles das threads criadas
    // into_iter(): criação de um iterator que adquire ownership de cada leitor
    let readers_handles: Vec<_> = readers.into_iter().map(|reader| {
        let rw_lock = rw_lock.clone();
        // criação das threads
        thread::spawn(move || {
            reader.read(&rw_lock);
        })
    // coleção de elementos (handles) que a thread::spawn retorna
    }).collect();

    // handles das threads criadas
    // into_iter(): criação de um iterator que adquire ownership de cada escritor
    let writers_handles: Vec<_> = writers.into_iter().map(|writer| {
        let rw_lock = rw_lock.clone();
        // criação das threads
        thread::spawn(move || {
            writer.write(&rw_lock);
        })
    // coleção de elementos (handles) que a thread::spawn retorna
    }).collect();

    // operação join() para cada handle retornado das threads 
    for handle in readers_handles {
        handle.join().unwrap();
    }

    // operação join() para cada handle retornado das threads
    for handle in writers_handles {
        handle.join().unwrap();
    }
}