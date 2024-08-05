#![allow(unused_variables)]

/// Experiments with lifetimes and ownership
fn main() {
    let v = vec![];//String::from("Hello"), String::from("world")];
    let default = String::from("Africa");
    let first = first_or(&v, &default);
    println!("{:?}", first);
}


fn first_or<'a>(strings: &'a Vec<String>, default: &'a String) -> &'a String {
    if strings.len() > 0 {
        &strings[0]
    } else {
        default
    }
}
