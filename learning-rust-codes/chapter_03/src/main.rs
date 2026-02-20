const GLOBAL_VARIABLE: i32 = 10; // Global variable

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

}
