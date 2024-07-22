#![allow(dead_code)]


use std::env;
use std::str::FromStr;


fn main() {
    // Parse
    let mut numbers = Vec::new();
    let args = env::args();
    for arg in args.skip(1) {
        numbers.push(u64::from_str(&arg).expect("Error parsing integers"));
    }

    // Check
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBERS ...");
        std::process::exit(1);
    }

    // Process
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("Greatest common divisor of {:?} is {}", numbers, d);
}


/// Computes the greatest common divisor of two integers
///
/// Uses Euclid’s algorithm.
/// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            (m, n) = (n, m);
        }
        m = m % n;
    }
    n
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(97, 9973), 1);
    assert_eq!(gcd(64, 96), 32);
}

