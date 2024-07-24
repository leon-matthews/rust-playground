#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).


// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// Note the `mut` modifier - we have taken ownership and changed string to `mut`
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);
    string_uppercase(data);
}
