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
        loop {
            if self.name == "Judith Butler" {
                println!("{} is THINKING.", self.name);
                thread::sleep(Duration::new(1, 0)); // Applies a 'simultaneity fudge factor'

                println!("{} is HUNGRY.", self.name);
                let _left = table.forks[self.right].lock().unwrap();
                let _right = table.forks[self.left].lock().unwrap();

                println!("{} is EATING.", self.name);
                thread::sleep(Duration::new(2, 0));
                println!("{} is DONE EATING.", self.name);
            } else {
                println!("{} is THINKING.", self.name);
                thread::sleep(Duration::new(1, 0)); // Applies a 'simultaneity fudge factor'

                println!("{} is HUNGRY.", self.name);
                let _left = table.forks[self.left].lock().unwrap();
                let _right = table.forks[self.right].lock().unwrap();

                println!("{} is EATING.", self.name);
                thread::sleep(Duration::new(2, 0));
                println!("{} is DONE EATING.", self.name);
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