
fn main() {
    // Order of definition does not matter
    fizzbuzz_to(100);

    // TODO:
    // https://doc.rust-lang.org/rust-by-example/fn/methods.html
}


/// Print correct 'fizzbuzz' response for the given integer.
fn fizzbuzz(number: i32) {
    if is_divisible_by(number, 15) {
        println!("Fizz Buzz");
    }
    else if is_divisible_by(number, 5) {
        println!("Buzz");
    }
    else if is_divisible_by(number, 3) {
        println!("Fizz");
    }
    else {
        println!("{number}");
    }
}


/// Run `fizzbuzz` on every positive integer up to `limit`, inclusive.
fn fizzbuzz_to(limit: i32) {
    for n in 1..=limit {
        fizzbuzz(n);
    }
}


/// Can the first number be divided cleanly by the second?
fn is_divisible_by(first: i32, second: i32) -> bool {
    // Avoid divide-by-zero
    if second == 0 {
        return false;
    }

    // Elegant!
    first % second == 0
}
