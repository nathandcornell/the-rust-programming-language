extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("********************");
    println!("* Guess The Number *");
    println!("********************");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess (1-100):");

        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Exactly right!! You are the winner!");
                break;
            },
        }
    }
}
