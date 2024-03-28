use rand::Rng;

pub fn generate_secret_number() -> u32 {
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
