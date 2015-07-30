extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("So?");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");
        println!("You say {}.", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need coffee.");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Yay!");
                break;
            }
        }
    }
}
