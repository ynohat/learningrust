use std::thread;

struct Thinker {
    name: String,
}

impl Thinker {
    fn new(name: &str) -> Thinker {
        Thinker {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} tucks in.", self.name);
        thread::sleep_ms(2000);
        println!("{} burps.", self.name);
    }
}

fn main() {
    let thinkers = vec![
        Thinker::new("Spinoza"),
        Thinker::new("Deleuze"),
        Thinker::new("Marx"),
        Thinker::new("Nietzsche"),
        Thinker::new("Foucault"),
    ];

    let handles: Vec<_> = thinkers.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
