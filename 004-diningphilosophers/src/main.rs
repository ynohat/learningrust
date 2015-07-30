use std::thread;
use std::sync::{Mutex, Arc};

struct Thinker {
    name: String,
    idx: usize,
}

impl Thinker {
    fn new(name: &str, idx: usize) -> Thinker {
        Thinker {
            name: name.to_string(),
            idx: idx,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.idx]
            .lock().unwrap();
        let _right = table.forks[(self.idx + 1) % table.forks.len()]
            .lock().unwrap();
        println!("{} tucks in.", self.name);
        thread::sleep_ms(10000);
        println!("{} burps.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let thinkers = vec![
        Thinker::new("Spinoza", 0),
        Thinker::new("Deleuze", 1),
        Thinker::new("Marx", 2),
        Thinker::new("Nietzsche", 3),
        Thinker::new("Foucault", 4),
    ];

    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });

    let handles: Vec<_> = thinkers.into_iter().map(|p| {
        let table = table.clone();
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
