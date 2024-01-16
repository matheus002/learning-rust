use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    //this block is the variables using to compose the text!!!
    let divisor_full = "********************************************************";
    let divisor_8x = "************";
    let game_title = "Guess the number!! ".bold().to_uppercase();
    let instructions = r#"
    ***********************************************
    **************  INSTRUCTIONS  *****************
        The ideia of the game is you to guess
        the secret number generate randomly in  
        the range of 1 to 101 with the mininum 
        of attemps as possible.
    ***********************************************
    ***********************************************
    **************  COMMANDS  *********************
            ENTER any number to validade your guess
            ENTER 'q' to quit the game.
            ENTER 'i' to show the instructions.
            ENTER 'a' to show the number of attemps
    ***********************************************
    "#;
    println!(
        "{}{}{}",
        "************* Welcome to ", game_title, divisor_8x
    );
    println!("{}\n\n", divisor_full);

    println!("{}", instructions);

    let mut attempts_number = 0;

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret number: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim() {
            "q" => {
                println!("You select to quit the game, bye...");
                break;
            }
            "i" => {
                println!("{}", instructions);
            }
            "a" => {
                println!(
                    "This is the attempt: {}",
                    attempts_number.to_string().bold()
                );
            }
            _ => (),
        };

        attempts_number += 1;
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Your guessed: {} was {}", guess, "Too Small".red())
            }
            Ordering::Greater => println!("Your guessed: {} was {}", guess, "Too big".red()),
            Ordering::Equal => {
                println!(
                    "Your guessed: {} was RIGTH {} with {} attempts",
                    guess,
                    "Congratulations! YOU WIN".green().bold(),
                    attempts_number.to_string().green().bold()
                );
                break;
            }
        }
    }
}
