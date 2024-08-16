#![allow(dead_code)]

/// Programming Rust (O'Reilly, 2nd Edition)
/// Chapter 4: Ownership and Moves

use std::rc::Rc;


fn main() {
    arguments_take_ownership();
    vectors_disallow_moves();
    consume_vector_contents();
    take_vector_contents();
    reference_counting();
    shared_reference();
    mutable_references();
}


/// Container's constructor takes ownership of its arguments
fn arguments_take_ownership() {
    // Create vector of composer
    #[derive(Debug)]
    struct Person {
        name: String,
        birth: i32,
    }
    let mut composers = Vec::new();
    let name = "Palestrina".to_string();
    let birth = 1525;
    composers.push(Person { name, birth });
    // `name` and `birth` are now invalid, their values belong to `composers`.

    println!("{composers:?}");
}


/// Vectors don't allow ownership of their contents to be taken away
fn vectors_disallow_moves() {
    // Create vector of integer strings
    let mut integers = Vec::new();
    for i in 101 .. 110 {
        integers.push(i.to_string());
    }

    // We cannot move value of of vector...
    // error[E0507]: cannot move out of index of `Vec<String>`
    //~ let third = integers[2];

    // ...but we can take a reference...
    let third = &integers[2];
    println!("We took a reference to {third:?}");

    // ...or a copy
    let fourth = integers[4].clone();
    println!("We cloned {fourth:?}");
}


/// Vectors allow themselves to be consumed
fn consume_vector_contents() {
    let v = vec!["Liberté", "Egalité", "Fraternité"];

    for s in v {
        let mut s = s.to_string();
        s.push('!');
        println!("{s}");
    }
    // `v` is now invalid! The for loop took ownership of its vector.
    //~ dbg!(&v);
}


/// Create vector of composer
fn take_vector_contents() {
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestrina".to_string()), birth: 1525 });
    println!("{:?}", &composers);

    // As per `vectors_disallow_moves()`, we can't take ownship of a name...
    // error[E0507]: cannot move out of index of `Vec<take_vector_contents::Person>`
    //~ let name = composers[0].name;

    // ...but because name is optional, we can use `Option.take()` and steal it
    let name = composers[0].name.take();
    print!("We took {:?}, ", name);
    println!("leaving None in its place: {:?}", composers);
}


/// Reference counting to manage lifetimes
fn reference_counting() {
    // All have type Rc<String> and share the same memory
    let s = Rc::new("Teddy".to_string());
    let t = s.clone();
    let u = s.clone();

    // Values owned by Rc pointers are immutable
    // error[E0596]: cannot borrow data in an `Rc` as mutable
    //~ u.push_str(" is a good boy");

    dbg!(s, t, u);
}


/// Shared references prevent changing of variable
fn shared_reference() {
    // Shared Reference to non-copy value
    //////////////////////////////////////
    #[allow(unused_mut)]
    let mut name = String::from("Leon");
    let shared_reference = &name;

    // Cannot rebind original name once ANY reference taken:
    //     error[E0506]: cannot assign to `name` because it is borrowed
    //~ name = "Matthews".to_string();

    // Cannot modify original's contents either:
    //     error[E0502]: cannot borrow `name` as mutable because it is also
    //     borrowed as immutable
    //~ name.push_str(" Matthews");

    // Use shared reference here to extend its lifetime
    println!("Shared reference is {}", shared_reference);
}


/// Mutable references take exclusive ownership
fn mutable_references() {
    let mut name = String::from("Leon");
    let mutable_reference = &mut name;

    // Cannot even READ variable while 'ref mute' has borrowed:
    //     error[E0502]: cannot borrow `name` as immutable because it is
    //     also borrowed as mutable
    //~ println!("Name variable is {}", name);

    mutable_reference.push('!');
    println!("Mutable reference is {}", mutable_reference);

    // Rust is smart enough to see that mute ref isn't used after above line
    // ie. its lifetime does not necessarily extend to enclosing scope
    println!("Name variable is {}", name);
}
