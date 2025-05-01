/// # Error Handling in Rust
/// 
/// This example demonstrates Rust's error handling mechanisms using Result, Option,
/// the ? operator, and custom error types.
/// Run this example with: `cargo run --bin error_handling`

use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::error::Error;
use std::fmt;

fn main() -> Result<(), Box<dyn Error>> {
    println!("RUST ERROR HANDLING EXAMPLE");
    println!("==========================\n");

    // ============= UNRECOVERABLE ERRORS WITH PANIC =============
    println!("=== Unrecoverable Errors with panic! ===");
    
    // Uncomment to see a panic in action
    // panic!("crash and burn");
    
    println!("Explicitly calling panic! (commented out in the code)");
    
    // ============= RECOVERABLE ERRORS WITH RESULT =============
    println!("\n=== Recoverable Errors with Result ===");
    
    // Result is defined as:
    // enum Result<T, E> {
    //     Ok(T),  // Success case with value of type T
    //     Err(E), // Error case with value of type E
    // }
    
    // Attempt to open a file that might not exist
    let file_result = File::open("hello.txt");
    
    // Handling the Result with match
    let _file = match file_result {
        Ok(file) => {
            println!("File opened successfully");
            file
        }
        Err(error) => {
            println!("Failed to open the file: {}", error);
            
            // We can match on specific error types
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("File not found, you could create it here");
                    // Uncomment to create the file:
                    // match File::create("hello.txt") {
                    //     Ok(fc) => fc,
                    //     Err(e) => panic!("Problem creating the file: {:?}", e),
                    // }
                    
                    // For demonstration, we'll just return a dummy file
                    File::open("Cargo.toml").expect("Cargo.toml should exist")
                }
                other_error => {
                    println!("Different error: {:?}", other_error);
                    panic!("Problem opening the file: {:?}", other_error);
                }
            }
        }
    };
    
    // ============= SHORTCUTS FOR PANIC ON ERROR =============
    println!("\n=== Shortcuts for panic! on Error ===");
    
    // unwrap() returns the value if Ok, or calls panic! if Err
    let _file = File::open("Cargo.toml").unwrap();
    println!("File opened with unwrap()");
    
    // expect() lets you choose the panic! error message
    let _file = File::open("Cargo.toml").expect("Failed to open Cargo.toml");
    println!("File opened with expect()");
    
    // ============= PROPAGATING ERRORS =============
    println!("\n=== Propagating Errors ===");
    
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    // Using the ? operator for shorter error propagation
    match read_username_from_file_short() {
        Ok(username) => println!("Username (short version): {}", username),
        Err(e) => println!("Error reading username (short version): {}", e),
    }
    
    // Even shorter with chaining and the ? operator
    match read_username_from_file_shortest() {
        Ok(username) => println!("Username (shortest version): {}", username),
        Err(e) => println!("Error reading username (shortest version): {}", e),
    }
    
    // ============= ERROR HANDLING WITH OPTION<T> =============
    println!("\n=== Error Handling with Option<T> ===");
    
    let numbers = vec![10, 20, 30];
    match get_element(&numbers, 1) {
        Some(value) => println!("Element at index 1: {}", value),
        None => println!("No element at that index"),
    }
    
    match get_element(&numbers, 5) {
        Some(value) => println!("Element at index 5: {}", value),
        None => println!("No element at that index"),
    }
    
    // Using the ? operator with Option (in Rust 2018+)
    match last_char_of_first_line("Hello\nWorld") {
        Some(c) => println!("First char of first line: {}", c),
        None => println!("No character found"),
    }
    
    // ============= CUSTOM ERROR TYPES =============
    println!("\n=== Custom Error Types ===");
    
    match parse_positive_integer("42") {
        Ok(n) => println!("Parsed positive integer: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    
    match parse_positive_integer("-5") {
        Ok(n) => println!("Parsed positive integer: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    
    match parse_positive_integer("not_a_number") {
        Ok(n) => println!("Parsed positive integer: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\nMain function returns Result<(), Box<dyn Error>>");
    println!("You can use the ? operator at the top level");
    let _file = File::open("Cargo.toml")?;
    println!("File opened with ? at the top level");
    
    Ok(())
}

// The long way to propagate errors
fn read_username_from_file() -> Result<String, io::Error> {
    // Try to open the file
    let file_result = File::open("username.txt");
    
    // Handle potential error opening the file
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    // File opened successfully, now read its contents
    let mut username = String::new();
    
    // Handle potential error reading the file
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Using the ? operator to propagate errors (shorter)
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // Return error if this fails
    let mut username = String::new();
    file.read_to_string(&mut username)?; // Return error if this fails
    Ok(username)
}

// Even shorter with method chaining and the ? operator
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
    
    // Equivalent to:
    // std::fs::read_to_string("username.txt")
}

// Safe way to get an element from a vector
fn get_element(vector: &[i32], index: usize) -> Option<i32> {
    if index < vector.len() {
        Some(vector[index])
    } else {
        None
    }
}

// Using ? with Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Custom error type
#[derive(Debug)]
enum ParseIntError {
    ParseError(std::num::ParseIntError),
    Negative,
}

impl fmt::Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseIntError::ParseError(e) => write!(f, "Failed to parse integer: {}", e),
            ParseIntError::Negative => write!(f, "Number cannot be negative"),
        }
    }
}

impl Error for ParseIntError {}

impl From<std::num::ParseIntError> for ParseIntError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseIntError::ParseError(err)
    }
}

// Function that uses custom error type
fn parse_positive_integer(s: &str) -> Result<i32, ParseIntError> {
    let num: i32 = s.parse()?; // This will convert std::num::ParseIntError to our custom error
    
    if num < 0 {
        return Err(ParseIntError::Negative);
    }
    
    Ok(num)
}