
use std::rc::Rc;


fn main() {
    println!("{}", string_maker_owned());
    println!("{}", string_maker_static());
    println!("{}", string_maker_shared());

    let mut s = String::new();
    string_maker_replace(&mut s);
    println!("{s}");
}

// FIVE WAYS to create a string, one of which doesn't work
///////////////////////////////////////////////////////////

// 1. Can't return a reference that will outlive its value
//~ fn string_maker_broken() -> &String {
    //~ let s = String::from("Hello world");
    //~ &s
//~ }

// 2. We can return an owned value
fn string_maker_owned() -> String {
    String::from("Hello World")
}

// 3. We can return a reference to a string literal, which has 'static lifetime
fn string_maker_static() -> &'static str {
    "Hello World"
}

// 4. Defer borrow-checking to runtime by using garbage collection
fn string_maker_shared() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}

// 5. Don't return a value, but mutate what was given
fn string_maker_replace(output: &mut String) {
    output.replace_range(.., "Hello world");
}
