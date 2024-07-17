#![allow(dead_code)]
#![allow(unused_variables)]

/// Programming Rust (O'Reilly, 2nd Edition)
/// Chapter 5: References
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;


fn main() {
    shared_and_mut_reference_args();
    dereference();
    references_to_references();
    comparing_references();
}


fn dereference() {
    let x = 10;
    let r = &x;

    // Dereference that reference!
    assert!(*r == 10);

    // The dot operator can implicity dereference
    let mut v = vec![2011, 1973, 2008, 1968];
    v.sort();                           // Implicity borrows mut& to v
    (&mut v).sort();                    // Same thing, explicitly stated
    println!("{:?}", v);

    // Unlike C++ references, we can reassign them in Rust
    let b = false;
    let x = 10;
    let y = 20;
    let mut r = &x;
    if b { r = &y }
    assert!(*r == 10 || *r == 20);
}


fn references_to_references() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1000, y: 678 };
    let r = &point;
    let rr = &r;
    let rrr = &rr;
    let rrr2: &&&Point = &&&point;      // Type not actually needed
    assert!(rrr2.y == 678);
}


/// Pass shared refereces and mutable referces as arguments to functions
fn shared_and_mut_reference_args() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "Many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "A salt cellar".to_string(),
        ],
    );

    show(&table);
    sort_works(&mut table);
    show(&table);
}

/// Pass table by shared reference
fn show(table: &Table) {
    // Note that because table is a shared reference iterating gives us shared
    // references to its contents.
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("    {}", work);
        }
    }
}


/// Pass in 'mute ref' to allow modification
fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}


/// Comparison operators 'see through' any number of references
fn comparing_references() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rx <= ry);
    assert!(rrx == rry);

    // But only when the types are the same on both sides
    // error[E0277]: can't compare `{integer}` with `&{integer}`
    //~ assert!(rx == rrx);
    assert!(rx == *rrx);        // &Point vs &Point thanks to de-ref

    // You *can* compare address if you really want
    assert!(rx == ry);
    assert!(!std::ptr::eq(rx, ry));
}
