#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::thread;


fn main() {
    borrow_immutable();
    borrow_mutably();
    move_into_thread();
    sorting_by_key();
}


fn borrow_immutable() {
    println!("borrow_immutable():");
    let list = vec![1, 2, 3];
    println!("  Before defining closure: {list:?}");
    let only_borrows = || println!("  From closure: {list:?}");
    println!("  Before calling closure: {list:?}");
    only_borrows();
    println!("  After calling closure: {list:?}");
}


fn borrow_mutably() {
    println!("borrow_mutably():");
    let mut list = vec![1, 2, 3];

    println!("  Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);

    // error[E0502]: cannot borrow `list` as immutable because it is also
    // borrowed as mutable
    //~ println!("Before calling closure: {list:?}");

    borrows_mutably();
    println!("  After calling closure: {list:?}");
}


fn move_into_thread() {
    println!("move_into_thread():");
    let list = vec![1, 2, 3];
    println!("  Before defining closure: {list:?}");
    thread::spawn(move || {
        println!("  From Thread: {list:?}");
    }).join().unwrap();

    // error[E0382]: borrow of moved value: `list`
    //~ println!("After defining closure: {list:?}");
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


/**
1. Move captured value out of the closure.
2. Mutate captured value
3. Neither move nor mutate
4. Capture nothing
*/
fn sorting_by_key() {
    let mut list = [
        Rectangle { width: 5, height: 5},
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
        Rectangle { width: 1, height: 1},
    ];

    // Captured mutably by closure to count number of calls made
    let mut num_operations = 0;

    // slice::sort_by_key() defined with FnMut trait bound, as it
    // calls the closure multiple times.
    list.sort_by_key(|r| {
        num_operations += 1; r.width;
    });
    println!("After {num_operations} calls to closure: {list:#?}");
}
