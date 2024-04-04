use rand::Rng;

fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

use std::io;
pub fn prompt_guess() -> u32 {
    println!("Please enter your guess (between 1 and 100):");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess.trim().parse().expect("Please enter a valid number!")
}
use std::cmp::Ordering;

fn check_guess(guess: u32, secret_number: u32) -> Ordering {
    guess.cmp(&secret_number)
}

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
