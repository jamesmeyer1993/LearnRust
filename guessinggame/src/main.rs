extern crate rand;  // see Cargo.toml

use std::io;            // read/write to io
use rand::Rng;          // for random numbers
use std::cmp::Ordering; // for match

fn main() {
    println!("Guess the number!");

    loop{
        // generate the number
        let r = rand::thread_rng().gen_range(1,101);

        // user enters a number
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");   // read in a string

        println!("You guessed: {}", guess);

        // convert guess to a u32
        //  Below, this is another example of a match expression. Make sure when doing this to use
        //  the keyword "match"!!!
        //  num is the value that gets returned to "guess" if there is a match with Ok
        //  "_" is a catch all
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // compare
        match guess.cmp(&r) {
            Ordering::Less => println!("Too small...The number is {}", r),
            Ordering::Greater => println!("Too big... The number is {}", r),
            Ordering::Equal => {
                println!("You guessed right! The number is {}", r);
                break;  // exit on a correct guess
            }
        }
    }
}
