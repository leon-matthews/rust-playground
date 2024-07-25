
fn main() {
    integer_overflow();
    let primes = sieve_of_eratosthenes();
    println!("There are {} primes less than 1,000,000", primes.len());
    println!("Vector has room for {}", primes.capacity());
    bools();
    characters();
    tuples();
    pointers();
    closures_are_variables();
    functions_are_variables(hello);
}


/// References, Boxes, and raw pointers
fn pointers() {
    // References
    //////////////
    // Enforce the single writer OR multiple readers rule.
    // `&T`     Immutable, shared reference. Read-only.
    // `&mut T` Mutable, exclusive reference.

    // Boxes
    /////////
    // Force allocation on the heap
    let t = (12_i32, "eggs");       // Type `(i32, &str)`
    let b = Box::new(t);        // Type `Box<(i32, &str)>`

    // Compare sizes on stack
    // On 64-bit Linux the actual values are 24 bytes and 8 bytes.
    // 8 bytes = Single pointer
    // 24 bytes = 8 for integer (4 plus padding), 8 for str pointer, 8 for length
    println!("Size of egg tuple is {} bytes", size_of_val(&t));
    println!("Size of `Box` of eggs is {} bytes", size_of_val(&b));

    // Automatic dereferences using dot operator
    println!("I have {} {}", (*b).0, (*b).1);
    println!("I have {} {}", b.0, b.1);
}


/// An n-tuple of assorted types
fn tuples() {
    // Create manually
    let tetracampeon = ("Brazil", 1994);    // Inferred type `(&str, i32)`
    println!("{} won the World Cup in {}", tetracampeon.0, tetracampeon.1);

    // Return two values from function, using pattern to destructure
    let (head, tail) = "I see the eigenvalue in thine eye".split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // The default function return value the `unit type` is an empty tuple: ()
}


/// Characters are four bytes long
fn characters() {
    // 'Sabi', rust in Japanese
    let rust = '錆';
    println!("Rust! {}!", rust);

    // Unicode escape is up to six hexadecimal characters
    // Only in range 0 -> 0xD7FF AND 0xE000 -> 0x10FFFF
    let disaproval = '\u{ca0}';
    println!("{}_{}", disaproval, disaproval);

    // Can be freely converted to integers
    assert_eq!(disaproval as u32, 0xCA0);
    assert_eq!(disaproval as u64, 3232);

    // But only u8 can be converted to chars, because wider types
    // include invalid values and conversions MUST succeed.
    assert_eq!(65 as char, 'A');

    // The stdlib provides a fallible function for this case
    assert_eq!(char::from_u32(65_u32), Some('A'));

    // Stdlib has lots of methods
    assert_eq!(rust.len_utf8(), 3);
    assert_eq!(rust.is_alphabetic(), true);
    assert_eq!(rust.is_digit(10), false);       // Digit in base-n
    assert_eq!(rust.escape_unicode().to_string(), "\\u{9306}");
}


/// Booleans are a single byte
fn bools() {
    // Comparison expressions evaluate to bools
    assert_eq!(3 > 2, true);
    assert_eq!(2 > 3, false);

    // You can cast booleans to integers, but not vice-versa.
    assert_eq!(true as u8, 1_u8);
    assert_eq!(false as u64, 0_u64);
}


/// Closures probably shouldn't be put into variables, but they can be
fn closures_are_variables() {
    let make_greeting = |who| format!("Hello, {}!", who);
    println!("{}", make_greeting("world"));
}


/// Plain function passed into `functions_are_variables()`
fn hello(who: &str) -> bool {
    println!("Hello, {}", who);
    false
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
/// As with C/C++ the type of the function matches its argument types
fn functions_are_variables(greeting: fn(&str) -> bool) {
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

    /*
    Count primes Sillyness, just practice using filters.

    The `with_capacity()` call we make below is NOT an optimisation worth
    making: we have to iterate over a million value to get the count needed.
    Secondly, the time saved by not having the vector resize was not
    measurable - even when hardcoded.
    */
    let num_primes = sieve.iter().filter(|p| **p).count();
    println!("Filter expression found {num_primes} true values in vector.");

    // Build vector of primes
    let mut primes: Vec<u32> = Vec::with_capacity(num_primes);
    for (number, is_prime) in sieve.iter().enumerate() {
        if *is_prime {
            primes.push(number as u32);
        }
    }

    primes
}
