
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


/// Message passing between threads using MPSC channels
fn main() {
    mpsc_create();
}


fn mpsc_create() {
    // Create pairs of Sender<T> and Receiver<T> types
    let (tx, rx) = mpsc::channel();

    // Send string from  `tx` to `rx`
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let greeting = String::from("Hello, world!");
        tx.send(greeting).unwrap();
    });

    // Block until message
    let received = rx.recv().unwrap();
    println!("Got {received:?}");
}
