// https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html

use rand::Rng; // rand gen crate
use std::cmp::Ordering;
use std::io; // standard library (prelude) // rand gen crate

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_number}");
    loop {
        println!("guess a number mate");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("don't know what you typed boss"); // .expect is error handling

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // error handling; if not int then loop is restarted
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // if statement
            Ordering::Less => println!("too small mate"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("well done mate");
                break;
            }
        }
    }
}
