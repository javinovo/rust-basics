extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        let read_chars = io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed {} chars: {}", read_chars, guess);

        let guess  = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue
        };

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }    
    }
}

// Improved in chapter 9

struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
    
    /* Getter: would only be required if this was in a different module (along with making the struct public)
    pub fn value(&self) -> u32 {
        self.value
    }*/
}