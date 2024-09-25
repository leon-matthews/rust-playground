
use std::thread;
use std::time::Duration;


fn main() {
    spawn_without_waiting();
    println!();
    wait_for_join();
    println!();
    closure_move();
}


/// Thread won't have time to finish
fn spawn_without_waiting() {
    // Thread!
    thread::spawn(|| {
        let limit = 10;
        for i in 1..=limit {
            println!("Thread {i} of {limit}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread
    let limit = 5;
    for i in 1..=limit {
        println!("Main {i} of {limit}");
        thread::sleep(Duration::from_millis(1));
    }
}


/// Waiting for the threads to finish Using join Handles
fn wait_for_join() {
    // Hold onto spawn's return value
    let handle = thread::spawn(|| {
        let limit = 10;
        for i in 1..=limit {
            println!("Thread {i} of {limit}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread
    let limit = 5;
    for i in 1..=limit {
        println!("Main {i} of {limit}");
        thread::sleep(Duration::from_millis(1));
    }

    // Block and wait until thread finishes
    handle.join().unwrap();
}


/// Closures can REALLY capture variables from their environment
fn closure_move() {
    let v = vec![1, 1, 2, 3, 5, 8];
    let handle = thread::spawn(move || {
        println!("Fibonacci, what!? {v:?}");
    });
    handle.join().unwrap();
}
