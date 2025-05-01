
fn main() {
    println!("Welcome to Rust Basics Learning Project!");
    println!("========================================\n");
    
    println!("This project demonstrates various Rust concepts through examples.");
    println!("Each example is designed to teach you about a specific Rust feature.");
    println!("\nAvailable examples:");
    println!("  1. Basic Types and Functions");
    println!("  2. Ownership and Borrowing");
    println!("  3. Structs and Enums");
    println!("  4. Error Handling with Result and Option");
    println!("  5. Concurrency and Threads");
    println!("\nRun individual examples using: cargo run --bin <example_name>");
    println!("Example: cargo run --bin basic_types");
    
    println!("\nAlternatively, you can explore the source code in src/examples/");
    println!("to learn about Rust concepts through code comments and documentation.");
    
    // For quick demonstration, we'll include a small Rust sample here
    println!("\n=== Quick Demonstration ===");
    
    // Example of variables and mutability
    let immutable = 10;
    // immutable = 15; // This would cause an error - immutable variables can't be changed
    
    let mut mutable = 10;
    mutable = 15; // This works because the variable is mutable
    println!("Mutability example: {}", mutable);
    
    // Example of a simple function
    println!("Function example: {}", add(5, 3));
    
    // String manipulation
    let name = "Rust";
    println!("String interpolation: Hello, {}!", name);
    
    // Vector (dynamic array)
    let mut languages = vec!["Rust", "C++", "Go"];
    languages.push("Python");
    println!("Vector example: {:?}", languages);
    
    // Match expression (pattern matching)
    let number = 7;
    match number {
        1 => println!("One!"),
        2..=5 => println!("Between 2 and 5"),
        6 | 7 | 8 => println!("6, 7, or 8 - got {}", number),
        _ => println!("Something else"),
    }
}

/// Adds two integers and returns their sum
///
/// # Arguments
/// * `a` - First integer
/// * `b` - Second integer
///
/// # Returns
/// Sum of the two integers
fn add(a: i32, b: i32) -> i32 {
    // In Rust, the last expression without a semicolon is the return value
    a + b
}
