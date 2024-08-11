
#![allow(unused_variables)]


fn main() {
    char_methods();
    string_methods();
    from_string();
    to_string();
}


/**
Chars are 32-bit Unicode code-points.
*/
fn char_methods() {
    let c = 'A';
    assert!(c.is_alphabetic());
    assert!(c.is_ascii());
    assert!( ! c.is_numeric());

    // ASCII characters can be cast to u8
    assert_eq!(c as u8, 65);

    // Many u32 values are valid unicode, but not all.
    assert_eq!(char::from_u32(0x1F980), Some('🦀'));
}


/**
Strings live on the heap and are resizable.

Can be dereferenced to `&str`, so can use methods from either str or String.
Methods are either imlemented on str on String depending on whether the method
needs a resizeable buffer or can use the text in place.

Represented under the covers as a `Vec<u8>` guarenteed to contain valid UTF-8.
*/
fn string_methods() {
    // Create
    let s = String::new();
    let s = String::with_capacity(1_024);
    let s = "Bizarre Bear Story".to_string();
    let s = "Borderlands"[1..6].to_owned();
    assert_eq!(s, String::from("order"));

    let words = vec!["Bizarre", "Bear", "Story"];
    let mut s: String = words.join(" ");
    println!("{s}");

    // Inspect
    assert_eq!(s.is_empty(), false);
    println!("length: {}, capacity {}", s.len(), s.capacity());
    assert_eq!(s.is_char_boundary(8), true);

    // Append and insert
    s.push('!');
    println!("{s}");
    s.push_str(" JFK? ");
    println!("{s}");
    s.extend(words);
    println!("{s}");
    println!("length: {}, capacity {}", s.len(), s.capacity());

    // `replace()` returns a new string
    let s2 = s.replace("BizarreBearStory", "");

    // As do the case-conversion methods
    let s3 = s.to_uppercase();
    println!("{s3}");
    let s4 = s.to_lowercase();
    println!("{s4}");

    // Remove
    s.clear();
    println!("length: {}, capacity {}", s.len(), s.capacity());
    s.shrink_to_fit();
    println!("length: {}, capacity {}", s.len(), s.capacity());
}


use std::str::FromStr;

/**
Parse other types from string.

If a type implements `FromStr`, then it provides a standard way to parse a value.

    pub trait FromStr: Sized {
        type Err;
        fn from_str(s: &str) -> Result<Self, Self::Err>;
    }

Don't forget, you gotta have `std::str::FromStr` is scope.
*/
fn from_string() {
    // Conversions are fast but quite fragile
    // `T::from_str()`
    assert_eq!(usize::from_str("123456789"), Ok(123456789));
    assert!(usize::from_str(" 123 ").is_err());

    assert_eq!(f64::from_str("123.321"), Ok(123.321));
    assert!(f64::from_str(" 123.321").is_err());

    assert_eq!(bool::from_str("true"), Ok(true));
    assert!(bool::from_str("TRUE").is_err());

    // `str::parse()`
    assert_eq!("true".parse::<bool>(), Ok(true));
    assert_eq!("42".parse::<u32>(), Ok(42));
}


/**
Convert non-textual values to strings.

*Display and ToString*

Implement `std::fmt::Display` trait for types with natural human-readable
printable form ot use the `{}` format specifier in the `format!` macro.

    pub trait Display {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
    }

Types that implements `Display` automatically get `ToString` implemented. It
provides only the `to_string()` method. The `ToString` trait predates `Display`
and is less flexible. You should prefer to implement `Display`.

*Debug*

ALL standard types, including containers, implement `Debug`, thanks to blanket
trait implementations. Usually it's best to let Rust `derive` an implementation.
*/
fn to_string() {
    // `Display` trait
    // Most basic stdlib types are Display, but not containers
    assert_eq!(format!("{} {} {}", true, 42, "Hello"), "true 42 Hello");

    // `ToString` trait
    assert_eq!((3e3).to_string(), "3000");

    // `Debug`
    assert_eq!(format!("{:?}", vec![2, 3, 5, 7]), "[2, 3, 5, 7]");
}
