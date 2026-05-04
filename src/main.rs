use rand::{self, RngExt};
use std::io;
fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Enter your guess: ");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("CMD read failed");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}", err);
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("Your guess: {guess}");
        if secret_number < guess {
            println!("Secret number is smaller than your guess");
        } else if secret_number > guess {
            println!("Secret number is greater than your guess");
        } else {
            println!("You guessed it!!!");
            break;
        }
    }
}
