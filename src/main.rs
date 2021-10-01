use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut counter = 0;
    println!("Guess a number between 1 and 100");
    loop {
        counter = counter + 1;
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess = guess.trim();
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a numeric value, try again!", guess);
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too small!", guess),
            Ordering::Greater => println!("{} is too big!", guess),
            Ordering::Equal => {
                println!(
                    "You win at you guess trial no {}!, {} == {}",
                    counter, guess, secret_number
                );
                break;
            }
        }
        println!("Try again");
    }
}
