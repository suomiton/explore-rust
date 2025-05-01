// Re-export all the WASM functionality from our example
pub mod examples;

// This is the main entry point for our library when compiled to WebAssembly
#[cfg(target_arch = "wasm32")]
pub use examples::wasm_example::*;