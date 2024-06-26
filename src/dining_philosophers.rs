use std::{
    sync::{Arc, Mutex},
    thread,
};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Forks {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, forks: &Forks) {
        // lock in the forks
        let _left = forks.forks[self.left].lock().unwrap();
        let _right = forks.forks[self.right].lock().unwrap();

        // eat
        println!("{} is eating", self.name);
        thread::sleep(std::time::Duration::from_millis(1000));

        // done
        println!("{} is done eating", self.name);
    }
}

fn main() {
    // set up the forks
    let forks = Arc::new(Forks {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    // set up the philosophers
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 4, 0),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            // bump up the counter by cloning the forks
            let forks = forks.clone();
            thread::spawn(move || {
                p.eat(&forks);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

// use std::thread;
// use std::sync::{Mutex, Arc};

// struct Philosopher {
//     name: String,
//     left: usize,
//     right: usize,
// }

// impl Philosopher {
//     fn new(name: &str, left: usize, right: usize) -> Philosopher {
//         Philosopher {
//             name: name.to_string(),
//             left: left,
//             right: right,
//         }
//     }

//     fn eat(&self, table: &Table) {
//         let _left = table.forks[self.left].lock().unwrap();
//         let _right = table.forks[self.right].lock().unwrap();

//         println!("{} is eating.", self.name);

//         thread::sleep_ms(1000);

//         println!("{} is done eating.", self.name);
//     }
// }

// struct Table {
//     forks: Vec<Mutex<()>>,
// }

// fn main() {
//     let table = Arc::new(Table { forks: vec![
//         Mutex::new(()),
//         Mutex::new(()),
//         Mutex::new(()),
//         Mutex::new(()),
//         Mutex::new(()),
//     ]});

//     let philosophers = vec![
//         Philosopher::new("Judith Butler", 0, 1),
//         Philosopher::new("Gilles Deleuze", 1, 2),
//         Philosopher::new("Karl Marx", 2, 3),
//         Philosopher::new("Emma Goldman", 3, 4),
//         Philosopher::new("Michel Foucault", 0, 4),
//     ];

//     let handles: Vec<_> = philosophers.into_iter().map(|p| {
//         let table = table.clone();

//         thread::spawn(move || {
//             p.eat(&table);
//         })
//     }).collect();

//     for h in handles {
//         h.join().unwrap();
//     }
// }
