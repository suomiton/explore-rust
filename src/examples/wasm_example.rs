/// # WebAssembly with Rust Example
///
/// This example demonstrates how to use Rust with WebAssembly.
/// To build this example:
/// 1. Install wasm-pack: `cargo install wasm-pack`
/// 2. Build the wasm package: `wasm-pack build --target web`
/// 3. Serve the directory (e.g., with `python -m http.server`)
/// 4. Open the wasm_example.html file in a browser
use wasm_bindgen::prelude::*;

// Import JavaScript functions from the browser
#[wasm_bindgen]
extern "C" {
    // Use the browser's console.log for debugging
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // Alert function from browser
    fn alert(s: &str);

    // Document object model functionality
    type Document;
    type Element;

    #[wasm_bindgen(js_namespace = document)]
    fn getElementById(id: &str) -> Element;

    #[wasm_bindgen(method, setter)]
    fn set_innerHTML(this: &Element, html: &str);

    #[wasm_bindgen(method, getter)]
    fn value(this: &Element) -> String;
}

// Helper macro for console logging (similar to println!)
macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

// Simple add function exported to JavaScript
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    console_log!("Rust: Adding {} and {}", a, b);
    a + b
}

// A more complex function that processes text
#[wasm_bindgen]
pub fn process_text(text: &str) -> String {
    console_log!("Rust: Processing text input: {}", text);

    // Simple text processing: count words, characters, and convert to uppercase
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    let uppercase = text.to_uppercase();

    format!(
        "Stats: {} words, {} characters\nUppercase: {}",
        word_count, char_count, uppercase
    )
}

// Function to calculate Fibonacci numbers
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    console_log!("Rust: Calculating Fibonacci number {}", n);

    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

// Update DOM elements directly from Rust
#[wasm_bindgen]
pub fn update_result() {
    console_log!("Rust: Updating DOM from Rust function");

    // Get input elements
    let input_element = getElementById("text-input");
    let input_text = input_element.value();

    // Process the input
    let result = process_text(&input_text);

    // Update the result element
    let result_element = getElementById("result");
    result_element.set_innerHTML(&result);
}

// Generate a sequence of Fibonacci numbers and update the DOM
#[wasm_bindgen]
pub fn generate_fibonacci_sequence(count: u32) {
    console_log!("Rust: Generating Fibonacci sequence of {} numbers", count);

    let mut sequence = String::from("<ul>");

    for i in 0..count {
        let fib = fibonacci(i);
        sequence.push_str(&format!("<li>Fibonacci({}) = {}</li>", i, fib));
    }

    sequence.push_str("</ul>");

    // Update the sequence element
    let sequence_element = getElementById("fibonacci-sequence");
    sequence_element.set_innerHTML(&sequence);
}

// Main entry point for the WebAssembly module
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_log!("Rust WebAssembly module initialized!");

    // Example direct DOM manipulation when the module loads
    let greeting_element = getElementById("wasm-greeting");
    greeting_element.set_innerHTML("Hello from Rust WebAssembly!");

    Ok(())
}
