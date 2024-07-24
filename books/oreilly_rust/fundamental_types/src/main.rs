
fn main() {
    integer_overflow();
    closures_are_variables();
    functions_are_variables(hello);

    let primes = sieve_of_eratosthenes();
    println!("There are {} primes less than 1,000,000", primes.len());
    println!("Vector has room for {}", primes.capacity())
}


fn closures_are_variables() {
    let make_greeting = |who| format!("Hello, {}!", who);
    println!("{}", make_greeting("world"));
}


/// Plain function passed into `functions_are_variables()`
fn hello(who: &str) {
    println!("Hello, {}", who);
}


/// Overflow behaviour is defined, and configurable
fn integer_overflow() {
    // Overflow will panic by default - but only in debug builds
    let mut i: u128 = 1;

    // Handle overflow, in any build
    loop {
        i = match i.checked_mul(2) {
            Some(i) => i,
            None => {
                break;
            }
        };
    }
    assert!(i < u128::MAX);
    println!("{i} < {}", u128::MAX);
}


/// Accepts the full-fledged function above as an argument (see `main()`)`
fn functions_are_variables(greeting: fn(&str)) {
    greeting("world!");
}


/// Use an array of bools to calculate primes
fn sieve_of_eratosthenes() -> Vec<u32> {
    let mut sieve = [true; 1_000_000];

    // Zero and one are not primes by definition
    sieve[0] = false;
    sieve[1] = false;

    // Run the sieve
    for i in 2..1_000 {
        if sieve[i] {
            let mut j = i * i;
            while j < 1_000_000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    // Count primes
    // Only to practice using filters. The `with_capacity()` call we make
    // below is NOT an optimisation. Firstly, we have to iterate over
    // a million value to get that value. Secondly, the time taken to
    // resize the primes vector was not measurable, even when hardcoded.
    let num_primes = sieve.iter().filter(|p| **p).count();
    dbg!(&num_primes);

    // Build vector of primes
    let mut primes: Vec<u32> = Vec::with_capacity(num_primes);
    for (number, is_prime) in sieve.iter().enumerate() {
        if *is_prime {
            primes.push(number as u32);
        }
    }

    primes
}
