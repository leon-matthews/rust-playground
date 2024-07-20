
/// Official Rust Book
///
/// Chapter 8 - Common Collections
/// https://doc.rust-lang.org/book/ch08-00-common-collections.html


fn strings() {
    // Create empty, append &str
    let mut s1 = String::new();
    s1.push_str("This one was empty!");
    println!("{s1}");

    // Create from &str, and char
    let mut s2 = "This was a &str type".to_string();
    s2.push('!');
    println!("{s2}");

    // Concatanate &str literals and strings, consuming first string.
    let s3 = s1 + " AND " + &s2;
    println!("{}", s3);

    // TODO: Continue from listing 8.18

    /// ALL strings are UTF-8
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
}
