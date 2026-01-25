use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // FIRST EXAMPLE
    println!("First part of the example:");

    // Immutable variables
    let x = 7;
    let y = 10;

    // An unused immutable variable example with _, to avoid warning.
    let _apples = 5;

    // How to print variables and expressions
    println!("x = {x} and y + 2 = {}", y + 2);

    println!("Please input your first guess.");
    let mut guess = String::new();

    // Input will be stored in the variable 'guess'
    // and we capture the Result of read_line in r1
    let r1 = io::stdin().read_line(&mut guess);
    println!("You guessed: {guess}");

    // Clear the previous input
    guess.clear();
    println!("Clear guess: '{guess}'");

    // We can reuse the variable 'guess' here and capture the Result of read_line in r2
    println!("Please input your second guess.");
    let r2 = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");

    // Print the Result values, which are of type std::io::Result<usize>
    println!("Read line results: r1 = {r1:?}, r2 = {r2}");

    // SECOND EXAMPLE
    println!("\nSecond part of the example:");
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("For debugging, the secret number is: {secret_number}");

    // Start an infinite loop
    loop {
        println!("Please input your guess. First try:");
        // Read the user's guess as a string input and store it in the mutable variable 'guess'
        // If reading the line fails, the program will crash and display "Failed to read line"
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
        // Here we use expect to handle potential parse errors by crashing the program with a message
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {guess}");

        // Compare the guess to the secret number and provide feedback
        // The cmp method returns an Ordering enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // Let's try it once more with error handling
        println!("Please input your guess. Second try:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
        // Here we handle potential parse errors using a match expression to provide user feedback without crashing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };
        println!("You guessed: {guess}");

        // Compare the guess to the secret number and provide feedback
        // The cmp method returns an Ordering enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
