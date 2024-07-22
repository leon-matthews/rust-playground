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

    // Process!
    let gcd = calculate_gcd_many(&numbers)
        .expect("Numbers must not be empty");
    println!("Greatest common divisor of {:?} is {}", numbers, gcd);
}


/// Computes the greatest common divisor of two integers
///
/// Uses Euclid’s algorithm.
/// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn calculate_gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            (m, n) = (n, m);
        }
        m = m % n;
    }
    n
}


/// Find GCD of all given numbers in given slice
fn calculate_gcd_many(numbers: &[u64]) -> Result<u64, String> {
    if numbers.len() == 0 {
        return Err(String::from("Cannot calculate GCD on empty slice"))
    }
    let mut divisor = numbers[0];
    for next in &numbers[1..] {
        divisor = calculate_gcd(divisor, *next);
    }
    Ok(divisor)
}


#[test]
fn test_calculate_gcd() {
    assert_eq!(calculate_gcd(97, 9973), 1);
    assert_eq!(calculate_gcd(64, 96), 32);
}

#[test]
fn test_calculate_gcd_many() {
    let numbers = [32, 64, 128];
    assert_eq!(calculate_gcd_many(&numbers).unwrap(), 32);

    let numbers = Vec::<u64>::new();
    let result = calculate_gcd_many(&numbers);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Cannot calculate GCD on empty slice");
}
