/// # Concurrency in Rust
/// 
/// This example demonstrates Rust's concurrency features: threads, message passing,
/// shared state, and synchronization primitives.
/// Run this example with: `cargo run --bin concurrency`

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::rc::Rc;

fn main() {
    println!("RUST CONCURRENCY EXAMPLE");
    println!("=======================\n");

    // ============= CREATING THREADS =============
    println!("=== Creating Threads ===");
    
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    // Main thread continues execution
    for i in 1..3 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(5));
    }
    
    // Wait for the spawned thread to finish
    handle.join().unwrap();
    
    // ============= THREAD WITH MOVE CLOSURE =============
    println!("\n=== Thread with Move Closure ===");
    
    // Create a vector in the main thread
    let v = vec![1, 2, 3];
    
    // Spawn a thread that uses the vector
    // The 'move' keyword transfers ownership to the new thread
    let handle = thread::spawn(move || {
        println!("Here's the vector from the spawned thread: {:?}", v);
    });
    
    // The following would cause a compile error, because v was moved into the thread
    // println!("Vector in main thread: {:?}", v);
    
    // Wait for the thread to finish
    handle.join().unwrap();
    
    // ============= MESSAGE PASSING BETWEEN THREADS =============
    println!("\n=== Message Passing Between Threads ===");
    
    // Create a channel
    let (tx, rx) = mpsc::channel();
    
    // Spawn a thread that sends messages
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            // Send the value through the channel
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive the messages in the main thread
    for received in rx {
        println!("Received: {}", received);
    }
    
    // ============= MULTIPLE PRODUCERS =============
    println!("\n=== Multiple Producers ===");
    
    // Create a new channel
    let (tx, rx) = mpsc::channel();
    
    // Clone the transmitter for the second thread
    let tx1 = tx.clone();
    
    // First thread - sends some values
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("thread"),
            String::from("1"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Second thread - sends other values
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("thread 2"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Main thread receives values from both sending threads
    for received in rx {
        println!("Received: {}", received);
    }
    
    // ============= SHARED STATE CONCURRENCY =============
    println!("\n=== Shared State Concurrency ===");
    
    // Create a mutex with an integer inside
    let m = Mutex::new(5);
    
    {
        // Lock the mutex to access the value
        let mut num = m.lock().unwrap();
        *num = 6;
        // Mutex is automatically unlocked when num goes out of scope
    }
    
    println!("m = {:?}", m);
    
    // ============= SHARING A MUTEX BETWEEN THREADS =============
    println!("\n=== Sharing a Mutex Between Threads ===");
    
    // Create a mutex with an integer
    // Wrap it in an Arc (Atomic Reference Counted) to safely share between threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // Spawn 10 threads that increment the counter
    for _ in 0..10 {
        // Clone the Arc to have a reference for each thread
        let counter = Arc::clone(&counter);
        
        // Spawn a thread that increments the counter
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Print the final value
    println!("Result: {}", *counter.lock().unwrap());
    
    // ============= SEND AND SYNC TRAITS =============
    println!("\n=== Send and Sync Traits ===");
    
    println!("Types implementing Send can be transferred between threads");
    println!("Types implementing Sync can be referenced from multiple threads");
    println!("Most types in Rust implement both, with some exceptions:");
    println!("  - Rc<T> implements neither Send nor Sync (use Arc<T> for thread safety)");
    println!("  - Cell and RefCell implement Send but not Sync");
    println!("  - MutexGuard implements neither Send nor Sync");
    
    // Example of a type that is not thread-safe
    let _local_rc = Rc::new(42);
    
    // This would cause a compile error - Rc cannot be sent between threads
    //thread::spawn(move || {
    //    println!("rc in thread: {}", *_local_rc);
    //});
    
    // Instead use Arc, which is thread-safe
    let thread_safe_rc = Arc::new(42);
    let thread_safe_rc_clone = Arc::clone(&thread_safe_rc);
    
    thread::spawn(move || {
        println!("Arc in thread: {}", *thread_safe_rc_clone);
    }).join().unwrap();
    
    println!("Arc in main thread: {}", *thread_safe_rc);
    
    // ============= SUMMARY =============
    println!("\n=== Summary ===");
    println!("1. Rust's ownership system helps prevent concurrency errors");
    println!("2. Thread spawning is explicit with thread::spawn");
    println!("3. Use message passing with channels for communication");
    println!("4. Use Mutex and Arc for shared state concurrency");
    println!("5. The compiler enforces thread safety through Send and Sync traits");
}