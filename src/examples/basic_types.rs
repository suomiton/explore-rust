/// # Basic Types and Functions in Rust
/// 
/// This example demonstrates Rust's basic types, variables, and functions.
/// Run this example with: `cargo run --bin basic_types`

fn main() {
    println!("RUST BASIC TYPES AND FUNCTIONS EXAMPLE");
    println!("======================================\n");

    // ============= VARIABLES AND MUTABILITY =============
    println!("=== Variables and Mutability ===");
    
    // By default, variables are immutable in Rust
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This would cause a compilation error!
    
    // To make a variable mutable, use the `mut` keyword
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; // This works because y is mutable
    println!("The value of y is now: {}", y);
    
    // Constants are always immutable and must have type annotations
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points is: {}", MAX_POINTS);
    
    // Shadowing allows reusing variable names
    let z = 5;
    let z = z + 1; // This creates a new variable that shadows the first one
    println!("The value of z is: {}", z);
    
    // Shadowing can also change types
    let spaces = "   "; // string type
    let spaces = spaces.len(); // number type
    println!("The number of spaces is: {}", spaces);
    
    // ============= SCALAR TYPES =============
    println!("\n=== Scalar Types ===");
    
    // Integers
    let a: i8 = -128; // signed 8-bit integer
    let b: u8 = 255;  // unsigned 8-bit integer
    let c: i32 = 2_147_483_647; // default integer type
    let d = 98_222; // Type will be inferred as i32
    println!("Integers: {}, {}, {}, {}", a, b, c, d);
    
    // Floating-point
    let e: f64 = 2.0; // Double precision float (default)
    let f: f32 = 3.0; // Single precision float
    println!("Floats: {}, {}", e, f);
    
    // Boolean
    let g: bool = true;
    let h: bool = false;
    println!("Booleans: {}, {}", g, h);
    
    // Character (Unicode scalar values)
    let i: char = 'z';
    let j: char = '😻'; // Rust supports Unicode
    let k: char = '∞';  // Even mathematical symbols
    println!("Characters: {}, {}, {}", i, j, k);
    
    // ============= COMPOUND TYPES =============
    println!("\n=== Compound Types ===");
    
    // Tuples - fixed length, can contain different types
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    // Destructuring a tuple
    let (x, y, z) = tup;
    println!("Tuple values: {}, {}, {}", x, y, z);
    // Accessing tuple elements with dot notation
    println!("Tuple access: {}, {}, {}", tup.0, tup.1, tup.2);
    
    // Arrays - fixed length, same type for all elements, allocated on stack
    let arr = [1, 2, 3, 4, 5];
    // Explicit type and size
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", 
                             "July", "August", "September", "October", "November", "December"];
    // Accessing array elements
    println!("First month: {}, Third element of arr: {}", months[0], arr[2]);
    
    // ============= VECTORS =============
    println!("\n=== Vectors ===");
    // Vectors - dynamic size, same type for all elements, allocated on heap
    let mut vec = Vec::new(); // empty vector
    vec.push(5);
    vec.push(6);
    vec.push(7);
    
    // Vector with initial values
    let vec2 = vec![1, 2, 3, 4, 5];
    
    println!("Vector: {:?}", vec);
    println!("Second element of vec2: {}", vec2[1]);
    println!("Vector length: {}", vec.len());
    
    // Iterating over vectors
    print!("Vector elements: ");
    for i in &vec {
        print!("{} ", i);
    }
    println!();
    
    // ============= STRINGS =============
    println!("\n=== Strings ===");
    
    // String literals (&str) - immutable, fixed-size string stored in the program's binary
    let string_literal = "Hello";
    
    // String type - growable, mutable, owned UTF-8 encoded string
    let mut string = String::from("Hello");
    string.push_str(", world!");
    println!("String: {}", string);
    
    // String operations
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note: s1 has been moved here and can no longer be used
    println!("Concatenated string: {}", s3);
    
    // String formatting with format!
    let s4 = format!("{} {} {}", "Rust", "is", "awesome");
    println!("Formatted string: {}", s4);
    
    // ============= FUNCTIONS =============
    println!("\n=== Functions ===");
    
    // Calling functions
    print_hello();
    print_number(42);
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // Expression vs Statement
    let y = {
        let x = 3;
        x + 1  // Note: no semicolon means this is an expression that returns a value
    };
    println!("The value of y is: {}", y);
    
    // ============= CONTROL FLOW =============
    println!("\n=== Control Flow ===");
    
    // If expressions
    let number = 6;
    if number < 5 {
        println!("Condition was true");
    } else if number == 5 {
        println!("Number is 5");
    } else {
        println!("Condition was false");
    }
    
    // If in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    
    // Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return a value from loop
        }
    };
    println!("The result of the loop is: {}", result);
    
    // While loop
    let mut number = 3;
    while number != 0 {
        print!("{}... ", number);
        number -= 1;
    }
    println!("LIFTOFF!");
    
    // For loop over a range
    print!("For loop over range: ");
    for number in 1..4 { // Range is exclusive of upper bound
        print!("{} ", number);
    }
    println!();
    
    print!("For loop over inclusive range: ");
    for number in 1..=3 { // Range includes upper bound
        print!("{} ", number);
    }
    println!();
    
    // For loop over a collection
    print!("For loop over array: ");
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        print!("{} ", element);
    }
    println!();
}

// Simple function with no return value
fn print_hello() {
    println!("Hello from a function!");
}

// Function with a parameter
fn print_number(x: i32) {
    println!("The number is: {}", x);
}

// Function with a return value
fn add(x: i32, y: i32) -> i32 {
    // In Rust, the last expression is implicitly returned
    // Note the absence of semicolon - this makes it an expression, not a statement
    x + y
    
    // Alternatively, we could use the return keyword:
    // return x + y;
}