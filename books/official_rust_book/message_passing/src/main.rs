
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


/// Message passing between threads using MPSC channels
fn main() {
    mpsc_create();
    multiple_messages();
}


/// Send messages from thread back to main
fn mpsc_create() {
    // Create pairs of Sender<T> and Receiver<T> types
    let (tx, rx) = mpsc::channel();

    // Send string from  `tx` to `rx`
    // Note that `tx` is moved out of the enclosing environment
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let greeting = String::from("Hello, world!");
        println!("{greeting}");
        tx.send(greeting).unwrap();

        //~ error[E0382]: borrow of moved value: `greeting`
        //~ println!("{greeting}");
    });

    // Block until message
    let received = rx.recv().unwrap();
    println!("Got {received:?}");
}


/// Have main wait for mulitple messages from thread
fn multiple_messages() {
    let (tx, rx) = mpsc::channel();

    // Thread
    thread::spawn(move || {
        let mut greeting = vec![
            "Hello",
            "from",
            "the",
            "thread",
        ];

        for part in greeting.drain(..) {
            tx.send(part).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Treat `rx` as iterator
    // Will block for values then exit when channel is closed.
    for part in rx {
        println!("{part}");
    }
}
