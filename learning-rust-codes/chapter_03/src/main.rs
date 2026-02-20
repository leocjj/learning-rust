const GLOBAL_VARIABLE: i32 = 10; // Global variable
                                 // let PI: f32 = 3.141598  // `let` cannot be used for global variables

fn main() {
    // Variables and Mutability
    // let x = 5; // Immutable variable, cannot be changed
    let mut x = 5; // Mutable variable, can be changed

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of the global variable is: {GLOBAL_VARIABLE}");
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    let x = 5; // First variable is shadowed by the second
    let x = x + 1; // Overshadows the first and is shadowed by the third inside the next block.
    {
        // inner scope
        let x = x * 2; // This creates a new variable x that shadows the previous one
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");

    // Primary scalar types:
    // integers, floating-point numbers, Booleans, and characters.
    let num1 = 43u16; // 16-bit unsigned integer
    let num2 = 3.14f64; // 64-bit floating-point number
    let is_active = true; // Boolean
    let letter = 'A'; // Character
    println!("The value of num1 is: {num1}");
    println!("The value of num2 is: {num2}");
    println!("The value of is_active is: {is_active}");
    println!("The value of letter is: {letter}");
}
