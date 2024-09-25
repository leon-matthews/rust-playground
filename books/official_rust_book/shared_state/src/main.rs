/*!
A mutex (mutual exclusion) allows only one thread to access data at a time.

Two rules:
1. You must attempt to acquire the lock before using the data.
2. You must unlock the data when you have finished.
*/

use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    mutex_single_threaded();
    mutex_multiple_threads();
}


/// Mutex API in a single thread
fn mutex_single_threaded() {
    let m = Mutex::new(6);

    {
        // `lock()` blocks until it's our turn, returns a `MutexGuard`
        let mut num = m.lock().unwrap();

        // `num` acts like a mutable reference to data *inside* mutex
        *num = 42;

        // The `num` MutexGuard goes out of scope and is dropped, lock released!
    }

    println!("Value is: {}", m.lock().unwrap());
}


/// Sharing a mutex between threads
fn mutex_multiple_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..100 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
