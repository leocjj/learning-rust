use rand::Rng;
use std::io;

fn main() {
    // FIRST EXAMPLE
    println!("First part of the example:");
    let x = 7;
    let y = 10;
    // An unsued variable example with _, to avoid warning.
    let _apples = 5; // immutable
    println!("x = {x} and y + 2 = {}", y + 2);

    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    let r1 = io::stdin().read_line(&mut guess);

    // We can reuse the variable 'guess' here
    // And capture the Result of read_line in r2
    let r2 = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: \n{guess}");
    println!("Read line results: r1 = {r1:?}, r2 = {r2}");

    guess.clear(); // Clear the previous input
    println!("Clear guess: '{guess}'");

    // Generate a random number between 1 and 100
    println!("Second part of the example:");

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");

    // SECOND EXAMPLE
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
