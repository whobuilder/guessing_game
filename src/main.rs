use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn read_input() -> String
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    input.trim().to_string()
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut counter = 0;
    println!("Guess a number between 1 and 100");
    loop {
        let guess = read_input();
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a numeric value, try again!", guess);
                continue;
            }
        };
        counter = counter + 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too small!", guess),
            Ordering::Greater => println!("{} is too big!", guess),
            Ordering::Equal => {
                println!(
                    "You win at your guess trial no {}!, {} == {}",
                    counter, guess, secret_number
                );
                println!("press a key to exit");
                read_input();  
                break;
            }
        }
        println!("Try again");
    }
}
