
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


/// Message passing between threads using MPSC channels
fn main() {
    mpsc_create();
    println!();
    multiple_messages();
    println!();
    multiple_producers();
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


/// Creating multiple producers by cloning the transmitter
fn multiple_producers() {
    // MPSC
    let (tx, rx) = mpsc::channel();

    // First thread
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals.iter().cycle() {
            tx2.send(val.clone()).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Second thread
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals.iter().cycle() {
            tx.send(val.clone()).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });


    // Receive messages
    for received in rx.into_iter().take(40) {
        println!("{received}");
    }
}