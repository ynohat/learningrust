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
    for t in &thinkers {
        t.eat();
    }
}
