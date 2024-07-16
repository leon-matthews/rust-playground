
fn main() {
    println!("{}", gcd(96, 128));
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

