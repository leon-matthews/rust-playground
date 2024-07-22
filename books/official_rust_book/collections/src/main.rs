#![allow(unused_variables)]

/// Official Rust Book
///
/// Chapter 8 - Common Collections
/// https://doc.rust-lang.org/book/ch08-00-common-collections.html

use std::collections::HashMap;


fn hash_maps() {
    let mut scores = HashMap::new();

    // I cannot *believe* that Rust can infer types from first insert
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 15);
    scores.insert(String::from("Yellow"), 50);

    // Map takes ownership
    let green = String::from("Green");
    scores.insert(green, 77);
    // error[E0382]: borrow of moved value: `green`
    //~ println!("{green}");

    // Panic if key not found?!
    // thread 'main' panicked at src/main.rs:22:23
    //~ scores[&String::from("Purple")];

    // Use `get()` method instead
    // We can use either &String or a literal &str,
    // and get an Option to a shared reference back, ie. Option<&i32>
    // Copy the contents to go from `Option<&i32>` to `Option<i32>`
    // The `unwrap_or()` method to get default on None.
    let score = scores.get("Blue").copied().unwrap_or(0);
    assert_eq!(score, 15);

    let score_not_found = scores.get("Purple").copied().unwrap_or(0);
    assert_eq!(score_not_found, 0);

    // Iterate over references (in arbitrary order).
    for (key, value) in &scores {
        println!("{} {}", *key, *value);
        println!("{} {}", key, value);      // Automatic dereference here
    }

    // Forget to iterate over reference? You'll take ownership of EVERYTHING!
    for (key, value) in scores {
        println!("{} {}", key, value);
    }
    // error[E0382]: borrow of moved value: `scores`
    //~ dbg!(&scores);

    // Adding a key and value only if a key NOT present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);     // New entry
    scores.entry(String::from("Blue")).or_insert(50);       // Nothing changed
    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    print!("{:?}", map);
}


fn strings() {
    // ALL strings are UTF-8
    let hellos = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];
    for hello in &hellos {
        println!("Hello? {hello}!");
    }

    // Create empty, append &str
    let mut s1 = String::new();
    s1.push_str("This one was empty!");
    println!("{s1}");

    // Create from &str, and char
    let mut s2 = "This was a &str type".to_string();
    s2.push('!');
    println!("{s2}");

    // Concatenate &str literals and strings, consuming first string.
    // Second string is actually coerced from &String to &str to match
    // signature of String's add method: `fn add(&self, &str) -> String`
    // This is called 'deref coercion'.
    let s3 = s1 + " AND " + &s2;
    println!("{}", s3);

    // Adding strings using the add operator gets unwieldy, fast:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    // Concatenate and format using `format!` macro, instead:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // You cannot index into a String, because of binary representation.
    // Nor can you safely slice it: Hitting half-way through UTF-8 character
    // will cause a Rust panic!
    let s1 = String::from("hello");
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    //~ let h = s1[0];

    // Iterate over characters...
    let hello = "Здравствуйте";
    for c in hello.chars() {
        print!("{c}.");
    }
    println!();

    // ...or bytes
    for b in hello.bytes() {
        print!("{b}.");
    }
    println!();
}


fn vectors() {
    // Let's create a vector containing a non-copyable types.
    let mut v = Vec::new();
    for i in 1..10 {
        v.push(format!("{i}{i}{i}{i}{i}"));
    }
    println!("Vector! {:?}", v);

    // Index notation can cause panic on out-of-bounds.
    let third = &v[2];
    println!("Dangerous access to {:?}", third);

    // Return `Option` using `get()` method
    let fourth = v.get(3);
    match fourth {
        Some(third) => println!("Checked access to {:?}", third),
        None => println!("There is no fourth element"),
    }

    // Iterate over references
    for i in &v {
        println!("{}", *i);
    }

    // Mutable references, but must be dereferenced to use
    let mut v = vec![10, 100, 1000];
    for i in &mut v {
        *i = *i / 2;
    }
    println!("Halved! {:?}", v);

    // Use an enum to store 'different' types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Integer(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Integer(42),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(3.14159),
    ];
    println!("Spreadsheet: {:#?}", row);
}


fn main() {
    vectors();
    strings();
    hash_maps();
}
