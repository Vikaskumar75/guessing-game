use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);

    println!("-------------------------------------------------");
    println!("Guess the number!");
    println!("-------------------------------------------------");

    loop {
        println!("\nPlease input your guess.");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().to_lowercase() == "quit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
