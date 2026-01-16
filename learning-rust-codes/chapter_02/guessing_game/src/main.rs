use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // An unsued variable example with _, to avoid warning.
    let _apples = 5; // immutable
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);

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
}
