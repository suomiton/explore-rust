/// # Ownership and Borrowing in Rust
/// 
/// This example demonstrates Rust's ownership system, which is one of its most unique
/// and powerful features. Ownership is how Rust manages memory without garbage collection.
/// Run this example with: `cargo run --bin ownership`

fn main() {
    println!("RUST OWNERSHIP AND BORROWING EXAMPLE");
    println!("===================================\n");

    // ============= OWNERSHIP BASICS =============
    println!("=== Ownership Basics ===");
    
    // In Rust, each value has exactly one owner
    // When the owner goes out of scope, the value is dropped
    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("String within scope: {}", s);
        
        // do stuff with s
    } // this scope is now over, and s is no longer valid
    
    // The following would cause a compile error because s is no longer valid:
    // println!("String after scope: {}", s);
    
    // ============= MOVE SEMANTICS =============
    println!("\n=== Move Semantics ===");
    
    let s1 = String::from("hello");
    // When we assign s1 to s2, the String data is "moved" to s2
    // s1 is no longer valid after this line
    let s2 = s1;
    
    // This would cause a compile error - s1 was moved:
    // println!("s1: {}", s1);
    
    // This works because s2 now owns the String:
    println!("s2: {}", s2);
    
    // This "move" behavior happens with data stored on the heap
    // Simple scalar values stored on the stack are copied, not moved
    let x = 5;
    let y = x; // x is still valid because i32 implements the Copy trait
    
    println!("x: {}, y: {}", x, y);
    
    // ============= CLONE FOR DEEP COPYING =============
    println!("\n=== Cloning (Deep Copying) ===");
    
    let s1 = String::from("hello");
    // If we want to copy the heap data, not just move ownership,
    // we can use the clone method
    let s2 = s1.clone();
    
    println!("s1: {}, s2: {}", s1, s2); // Both are valid
    
    // ============= OWNERSHIP AND FUNCTIONS =============
    println!("\n=== Ownership and Functions ===");
    
    let s = String::from("hello");
    
    takes_ownership(s);
    // s is no longer valid here because its ownership was transferred to the function
    
    let x = 5;
    makes_copy(x);
    // x is still valid here because i32 implements Copy
    println!("x is still valid: {}", x);
    
    // Functions can also return ownership
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("s1 received from function: {}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into the function and the function returns a value that moves into s3
    println!("s3 received after passing s2 to function: {}", s3);
    
    // ============= REFERENCES AND BORROWING =============
    println!("\n=== References and Borrowing ===");
    
    // Instead of taking ownership, functions can borrow values using references
    let s1 = String::from("hello");
    
    // &s1 creates a reference to s1 but doesn't take ownership
    let len = calculate_length(&s1);
    
    println!("The length of '{}' is {}.", s1, len);
    // s1 is still valid here because it was only borrowed, not moved
    
    // ============= MUTABLE REFERENCES =============
    println!("\n=== Mutable References ===");
    
    let mut s = String::from("hello");
    
    change(&mut s); // Pass a mutable reference
    println!("After change: {}", s);
    
    // Rule: You can have only ONE mutable reference to a particular piece of data in a particular scope
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    // let r2 = &mut s; // This would cause a compile error - can't have two mutable references
    
    println!("r1: {}", r1);
    
    // After r1 is no longer used, we can create a new mutable reference
    let r2 = &mut s;
    println!("r2: {}", r2);
    
    // Rule: You cannot have a mutable reference while you have immutable ones
    let mut s = String::from("hello");
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // OK - r1 and r2 are no longer used
    println!("r3: {}", r3);
    
    // ============= DANGLING REFERENCES =============
    println!("\n=== Preventing Dangling References ===");
    
    // Rust prevents dangling references at compile time
    // This function would cause a compile error:
    //
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s // We would return a reference to s, but s goes out of scope at the end of the function
    //        // so this reference would be pointing to invalid memory
    // }
    
    // Instead, return the String directly:
    let s = no_dangle();
    println!("No dangle: {}", s);
    
    // ============= THE SLICE TYPE =============
    println!("\n=== Slices ===");
    
    // Slices let you reference a contiguous sequence of elements in a collection
    let s = String::from("hello world");
    
    let hello = &s[0..5];  // or &s[..5]
    let world = &s[6..11]; // or &s[6..]
    
    println!("First word: {}, Second word: {}", hello, world);
    
    // Function that uses string slices
    let s = String::from("hello world");
    let first = first_word(&s);
    println!("First word using function: {}", first);
    
    // String literals are slices
    let s = "Hello, world!"; // s is a &str
    println!("String literal (which is a slice): {}", s);
    
    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Array slice: {:?}", slice);
    
    // ============= SUMMARY =============
    println!("\n=== Summary ===");
    println!("1. Each value in Rust has a single owner");
    println!("2. When the owner goes out of scope, the value is dropped");
    println!("3. You can transfer ownership (move), clone, or borrow values");
    println!("4. References allow you to use values without taking ownership");
    println!("5. References are guaranteed to point to valid data due to Rust's borrow checker");
}

// Takes ownership of the String parameter
fn takes_ownership(some_string: String) {
    println!("Function took ownership of: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called

// Makes a copy of the i32 parameter
fn makes_copy(some_integer: i32) {
    println!("Function made a copy of: {}", some_integer);
} // Here, some_integer goes out of scope but nothing special happens

// Gives ownership of its return value
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // returned and moves ownership to the calling function
}

// Takes ownership and gives it back
fn takes_and_gives_back(a_string: String) -> String {
    a_string // returns a_string and moves ownership to the calling function
}

// Borrows a reference to String but doesn't take ownership
fn calculate_length(s: &String) -> usize {
    s.len() // s is a reference to a String
} // Here, s goes out of scope, but it doesn't have ownership, so nothing happens

// Changes a string through a mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Returns a String directly to avoid dangling references
fn no_dangle() -> String {
    let s = String::from("hello");
    s // ownership is moved out, so no problem
}

// Uses string slices for greater flexibility
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}