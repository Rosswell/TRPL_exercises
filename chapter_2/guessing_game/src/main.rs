extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    // creates an rng thread from the rand package
    // uses the thread to create a random number from 1-100 (Rng::gen_range())
    let sn = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");

        // mutable, new, empty String
        let mut guess = String::new();

        // reading the line from stdin, assigning it to the guess reference
        // expect prints the String param if the read_line fxn results in an Err
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // x: u32 casts x to an unsigned 32 bit int
        // trim trims front and back (eliminating the \n)
        // parse turns it into a string, then error handling
        let guess: u32 = match guess.trim().parse() {
            // a successful parse() literally results in an Ok value that contains the resulting number
            Ok(num) => num,
            // this catches an Err containing anything, as parsing can prolly result in multiple types of errors
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match is kinda a combination of regex matching and a switch statement
        match guess.cmp(&sn) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
