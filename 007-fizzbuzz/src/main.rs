use std::thread;

fn main() {
    let mut number = -1;
    loop {
        number = number + 1;
        let fizz = number % 3 == 0;
        let buzz = number % 5 == 0;
        if fizz || buzz {
            if fizz {
                print!("fizz");
            }
            if buzz {
                print!("buzz");
            }
        } else {
            print!("{}", number);
        }
        print!("\n");
        thread::sleep_ms(300);
    }
}
