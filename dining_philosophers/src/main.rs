use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

// struct que representa uma mesa com vários garfos
struct Table {
    forks: Vec<Mutex<()>>,
}

// struct que representa os filósofos
struct Philosopher {
    name: String,
    // usize: tipo utilizado para indexar vetores (Table.forks)
    left: usize,
    right: usize,
}

// implementação da struct Philosopher
impl Philosopher {

    // função new: convenção para criação de instâncias
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        loop {
            // Judith Butler pega o garfo direito e esquerdo (nesta equência)
            if self.name == "Judith Butler" {
                println!("{} is THINKING.", self.name);
                // simulação do tempo que o filósofo gasta pensando
                thread::sleep(Duration::new(1, 0));

                println!("{} is HUNGRY.", self.name);
                // filósofo tenta pegar os garfos
                let _left = table.forks[self.right].lock().unwrap();
                let _right = table.forks[self.left].lock().unwrap();

                println!("{} is EATING.", self.name);
                // simulação do tempo que o filósofo gasta comendo
                thread::sleep(Duration::new(2, 0));
                println!("{} is DONE EATING.", self.name);
            // outros filósofos pegam o garfo esquerdo e direito (nesta sequência)
            } else {
                println!("{} is THINKING.", self.name);
                // simulação do tempo que o filósofo gasta pensando
                thread::sleep(Duration::new(1, 0));

                println!("{} is HUNGRY.", self.name);
                // filósofo tenta pegar os garfos
                let _left = table.forks[self.left].lock().unwrap();
                let _right = table.forks[self.right].lock().unwrap();

                println!("{} is EATING.", self.name);
                // simulação do tempo que o filósofo gasta comendo
                thread::sleep(Duration::new(2, 0));
                println!("{} is DONE EATING.", self.name);
            }
        }
    }

}

fn main() {
    // ARC: atomic reference count: para compartilhar a table entre múltiplas threads
    // Mutex: forma de controlar a concorrência
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    // criação dos filósofos com seus respectivos nomes e garfos
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("John Locke", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    // handles das threads criadas
    // into_iter(): criação de um iterator que adquire ownership de cada filósofo
    let handles: Vec<_> = philosophers.into_iter().map( |philosopher| {
        let table = table.clone();
        // criação das threads
        thread::spawn(move || {
            philosopher.eat(&table);
        })
    // coleção de elementos (handles) que a thread::spawn retorna
    }).collect();

    // operação join() para cada handle retornado das threads
    for handle in handles {
        handle.join().unwrap();
    }
}