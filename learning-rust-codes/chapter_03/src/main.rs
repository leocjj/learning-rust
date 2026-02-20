use std::io;

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

    let num1: u16 = 43; // 16-bit unsigned integer
    println!("The value of num1 is: {num1}");
    let num1 = 43u16; // 16-bit unsigned integer, shadows the previous num1
    println!("The value of num1 is: {num1}");

    let mut int1: u8 = 254; // 8-bit unsigned integer
                            // with 255, this will cause an overflow, wrapping around to 0 if cargo build --release
                            // is used, otherwise it will panic in debug mode.
    int1 = int1 + 1;
    println!("The value of int1 after overflow is: {int1}");

    let mut bin1 = 0b1010; // Binary literal for 10
    bin1 >>= 1; // Right shift by 1, effectively dividing by 2
    println!("The value of bin1 is: {bin1}");

    let mut sum1 = 0;
    for i in 1..=5 {
        sum1 += i; // sum1 = sum1 + i
    }
    println!("The sum of numbers from 1 to 5 is: {sum1}");

    let bool1 = true;
    let bool2: bool = false;
    println!("The value of bool1 is: {bool1}");
    println!("The value of bool2 is: {bool2}");

    // char type is 4 bytes in size and represents a Unicode scalar value,
    // which means it can represent a lot more than just ASCII.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of the tuple is: {:?}", tup);
    println!("The value of the tuple is: {:#?}", tup);
    println!("The value of the first element of the tuple is: {}", tup.0);

    // pattern matching to destructure a tuple value
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];
    println!("The value of the array is: {:?}", a);
    println!("The value of the array is: {:#?}", a);
    println!("The value of the first element of the array is: {}", a[0]);
    println!("The value of the array is: {:?}", b);
    println!("The value of the first element of the array is: {}", b[0]);
    println!("The value of the array is: {:?}", c);
    println!("The value of the first element of the array is: {}", c[0]);

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

    // The following line will cause a panic at runtime if the user enters an index
    // that is out of bounds for the array `a`.
    // Rust protects you against this kind of error by immediately exiting instead of
    // allowing the memory access and continuing.
}
