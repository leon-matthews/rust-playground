#![allow(dead_code)]

use std::thread;
use std::time::Duration;


fn main() {
    //~ spawn_thread();
    move_closures();
}


/// Spawn second thread, then wait for it to finish
fn spawn_thread() {
    // Spawn second thread and start counting
    let handle = thread::spawn(|| {
        for i in 1..=6 {
            println!("Spawned thread says {}", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Carry on in main thread
    for i in 1..=3 {
        println!("Main  thread says {}", i);
        thread::sleep(Duration::from_millis(200));
    }

    // Wait for spawned thread to finish
    println!("Main thread waiting...");
    handle.join().unwrap();
    println!("Everybody finished");
}


fn move_closures() {
    let v = vec![1, 2, 3, 4, 5];

    // Force closure to take ownership of vector
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}
