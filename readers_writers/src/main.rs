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
            if let Ok(mut write_guard) = rw_lock.write() {
                *write_guard += 1;
                println!("{} UPDATED value to {}", self.name, *write_guard);
            }
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
            if let Ok(read_guard) = rw_lock.read() {
                println!("{} READ value {}", self.name, *read_guard);
            }
            thread::sleep(Duration::from_millis(500));
        }
    }

}

fn main() {
    let value:i32 = 0;
    let rw_lock = Arc::new(RwLock::new(value));

    let readers = vec![
        Reader::new("Judith Butler"),
        Reader::new("Gilles Deleuze"),
        Reader::new("Karl Marx"),
        Reader::new("John Locke"),
        Reader::new("Michel Foucault"),
    ];

    let writers = vec![
        Writer::new("Socrates"),
        Writer::new("Plato"),
        Writer::new("Aristotle"),
    ];

    let readers_handles: Vec<_> = readers.into_iter().map( |reader| {
        let rw_lock = rw_lock.clone();

        thread::spawn(move || {
            reader.read(&rw_lock);
        })

    }).collect();

    let writers_handles: Vec<_> = writers.into_iter().map( |writer| {
        let rw_lock = rw_lock.clone();

        thread::spawn(move || {
            writer.write(&rw_lock);
        })

    }).collect();

    for handle in readers_handles {
        handle.join().unwrap();
    }
    for handle in writers_handles {
        handle.join().unwrap();
    }
}