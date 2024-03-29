use std::io;
pub fn prompt_guess() -> u32 {
    println!("Please enter your guess (between 1 and 100):");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess.trim().parse().expect("Please enter a valid number!")
}

mod secret_number;
mod user_input;
mod guess_checking;

use secret_number::generate_secret_number;
use user_input::prompt_guess;
use guess_checking::check_guess;
use std::cmp::Ordering;

fn print_result(result: Ordering, secret_number: u32) {
    match result {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("Congratulations! You guessed the number correctly: {}", secret_number),
    }
}


fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = generate_secret_number();

    loop {
        let guess = prompt_guess();

        let result = check_guess(guess, secret_number);

        print_result(result, secret_number);

        if result == Ordering::Equal {
            break;
        }
    }
}
