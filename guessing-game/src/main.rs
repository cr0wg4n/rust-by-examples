use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("This is your guess: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                println!("Too Big!");
            },
            Ordering::Less => {
                println!("Too Small!");
            },
            Ordering::Equal =>{
                println!("You Win!!!");
                break;
            }
        }
    }
}
