
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
