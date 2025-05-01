/// # Structs and Enums in Rust
/// 
/// This example demonstrates Rust's structs (similar to classes in other languages)
/// and enums (sum types), which form the basis for custom data types in Rust.
/// Run this example with: `cargo run --bin structs_enums`

// Using the Debug trait for printing structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block for Rectangle
impl Rectangle {
    // Associated function (like a static method)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method with &self reference (like instance method)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that takes a reference to another Rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// A struct with lifetime parameters
struct ImportantExcerpt<'a> {
    part: &'a str, // This is a reference that needs a lifetime parameter
}

// Define an enum for IP address types
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Enum with associated data of different types
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// More complex enum with different types of data for each variant
#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Anonymous struct
    Write(String),              // String
    ChangeColor(i32, i32, i32), // Three i32 values
}

// Implementing methods on an enum
impl Message {
    fn call(&self) {
        // Method body would define behavior based on Message type
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to: rgb({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    println!("RUST STRUCTS AND ENUMS EXAMPLE");
    println!("=============================\n");

    // ============= STRUCTS BASICS =============
    println!("=== Structs Basics ===");
    
    // Creating a struct instance
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Rectangle: {:?}", rect1); // :? uses Debug formatting
    println!("Rectangle (pretty): {:#?}", rect1); // :#? uses pretty Debug formatting
    
    // Using a method
    println!("Area of rectangle: {} square pixels", rect1.area());
    
    // Using an associated function (static method)
    let rect2 = Rectangle::new(10, 20);
    println!("New rectangle: {:?}", rect2);
    
    // Using another method that takes a parameter
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    
    // ============= STRUCT UPDATE SYNTAX =============
    println!("\n=== Struct Update Syntax ===");
    
    let rect3 = Rectangle {
        width: rect1.width,
        height: 10,
    };
    println!("Rectangle with explicit fields: {:?}", rect3);
    
    // Struct update syntax
    let rect4 = Rectangle {
        width: 5,
        ..rect1 // Use the rest of the values from rect1
    };
    println!("Rectangle with update syntax: {:?}", rect4);
    
    // ============= TUPLE STRUCTS =============
    println!("\n=== Tuple Structs ===");
    
    // Tuple structs have unnamed fields but are a distinct type
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // ============= UNIT-LIKE STRUCTS =============
    println!("\n=== Unit-like Structs ===");
    
    // Unit-like structs have no fields
    struct AlwaysEqual;
    
    let subject = AlwaysEqual;
    println!("Unit-like struct created (no fields to display)");
    
    // ============= STRUCTS WITH LIFETIMES =============
    println!("\n=== Structs with Lifetimes ===");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap_or("");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: '{}'", excerpt.part);
    
    // ============= ENUMS BASICS =============
    println!("\n=== Enums Basics ===");
    
    // Using simple enum variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IP types: {:?} and {:?}", four, six);
    
    // Using enum with associated data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("IP addresses: {:?} and {:?}", home, loopback);
    
    // Using complex enum
    let quit_message = Message::Quit;
    let move_message = Message::Move { x: 10, y: 20 };
    let write_message = Message::Write(String::from("Hello, Rust!"));
    let color_message = Message::ChangeColor(255, 0, 255);
    
    // Calling method on different enum variants
    quit_message.call();
    move_message.call();
    write_message.call();
    color_message.call();
    
    // ============= OPTION ENUM =============
    println!("\n=== Option Enum ===");
    
    // Option is a standard library enum for values that can be null
    // defined as:
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);
    
    // Unwrapping an Option
    println!("Unwrapped number: {}", some_number.unwrap());
    // println!("Unwrapped absent: {}", absent_number.unwrap()); // This would panic!
    
    // Safer ways to handle Option
    let value = match some_number {
        Some(num) => num,
        None => 0, // Default value if None
    };
    println!("Safely unwrapped with match: {}", value);
    
    let value = some_number.unwrap_or(0); // Provide default value
    println!("Unwrapped with default: {}", value);
    
    // ============= MATCH CONTROL FLOW =============
    println!("\n=== Match Control Flow ===");
    
    // Match is exhaustive - all possibilities must be covered
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Coin value: {}", value_in_cents(coin));
    
    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    
    // Match with catch-all pattern
    let some_value = 0u8;
    match some_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => println!("something else: {}", some_value),
    }
    
    // ============= IF LET =============
    println!("\n=== if let ===");
    
    // if let is a shorter way to handle one match case and ignore the rest
    let some_value = Some(3);
    
    // Using match
    match some_value {
        Some(3) => println!("Match: three!"),
        _ => (), // Do nothing for other cases
    }
    
    // Using if let (more concise when you only care about one pattern)
    if let Some(3) = some_value {
        println!("if let: three!");
    }
    
    // Example with else
    let mut count = 0;
    if let Coin::Quarter(state) = Coin::Quarter(UsState::Alaska) {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

// Enum for the match control flow example
#[derive(Debug)]
enum UsState {
    Alabama, Alaska, Arizona, Arkansas, /* ... etc */
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Enum with associated data
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}