use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        // place while true here
        loop {
            if self.name == "Judith Butler" {
                let _left = table.forks[self.right].lock().unwrap();
                thread::sleep(Duration::new(1, 0)); // Applies a 'simultaneity fudge factor'
                let _right = table.forks[self.left].lock().unwrap();
                println!("{} is eating.", self.name);
                thread::sleep(Duration::new(1, 0));
                println!("{} is done eating.", self.name);
            } else {
                let _left = table.forks[self.left].lock().unwrap();
                thread::sleep(Duration::new(1, 0)); // Applies a 'simultaneity fudge factor'
                let _right = table.forks[self.right].lock().unwrap();
                println!("{} is eating.", self.name);
                thread::sleep(Duration::new(1, 0));
                println!("{} is done eating.", self.name);
            }
        }
    }
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("John Locke", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map( |philosopher| {
        let table = table.clone();

        thread::spawn(move || {
            philosopher.eat(&table);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}