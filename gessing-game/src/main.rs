use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("WELCOME to the Guess the number!!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret number: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid Input".red());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "Congratulations! YOU WIN!!!!".green());
                break;
            }
        }
    }
}
