struct Thinker {
    name: String,
}

impl Thinker {
    fn new(name: &str) -> Thinker {
        Thinker {
            name: name.to_string(),
        }
    }
}

fn main() {
    let p1 = Thinker::new("Spinoza");
    let p2 = Thinker::new("Deleuze");
    let p3 = Thinker::new("Marx");
    let p4 = Thinker::new("Nietzsche");
    let p5 = Thinker::new("Foucault");
}
