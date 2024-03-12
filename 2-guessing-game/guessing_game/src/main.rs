use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number!");

    // NOTE: gen the random num
    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess:");
        println!("The secret number is: {}", secret_number);

        let mut guess = String::new();

        // NOTE: read the user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        print!("You guessed: {}", guess);

        // NOTE: shadowing -> when we declare var with the same name, but diff type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Pls type a number");
                continue;
            }
        };

        // NOTE: cmp(..) - meaning comparing
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!!".red()),
            Ordering::Equal => {
                println!("{}", "You WIN!!!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big!!!".red()),
        }
    }
}
