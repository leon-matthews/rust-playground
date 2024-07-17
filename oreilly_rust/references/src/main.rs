#![allow(dead_code)]
#![allow(unused_variables)]

/// Programming Rust (O'Reilly, 2nd Edition)
/// Chapter 5: References
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;


fn main() {
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
