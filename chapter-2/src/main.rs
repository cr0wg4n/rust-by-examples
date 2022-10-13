use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number!");

        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = guess.trim().parse().expect("Please type a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("to small!"),
            Ordering::Greater => println!("to big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
        }

        println!();
    }

    println!("\nThe secret number was: {secret_number}");
}
