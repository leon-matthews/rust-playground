#![allow(dead_code)]
#![allow(unused_variables)]

/// Programming Rust (O'Reilly, 2nd Edition)
/// Chapter 6: Expressions


fn main() {
    // Most of this chapter is straight-forward, but closure's syntax is new
    closures();
}


fn closures() {
    let is_even = |x| x % 2 == 0;

    // Block required if return type specified explicitly
    let is_even_explicit = |x: u64| -> bool { x % 2 == 0 };

    println!("{}", is_even(4));
    println!("{}", is_even_explicit(4545443));
}
