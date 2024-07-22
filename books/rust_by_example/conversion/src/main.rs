#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;



/// Conversion
/// Rust by Example, Chapter 6
/// https://doc.rust-lang.org/rust-by-example/conversion.html


// From and Into
/////////////////

#[derive(Debug)]
struct Number {
    value: i32,
}


/// From trait to create custom constructor, consuming its argument
impl From<i32> for Number {
    fn from(item: i32) -> Number {
        Number { value: item }
    }
}

impl From<u128> for Number {
    fn from(item: u128) -> Number {
        Number { value: item as i32 }
    }
}


/// A family of conversion functions from out type `Into` others'.
impl Into<u64> for Number {
    fn into(self) -> u64 {
        self.value as u64
    }
}

impl Into<u128> for Number {
    fn into(self) -> u128 {
        self.value as u128
    }
}


// TryFrom and TryInto
///////////////////////

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);


impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}



// To and From Strings
///////////////////////


fn main() {
    // From and Into
    /////////////////

    // str to String
    let my_str = "hello";
    let my_string = String::from(my_str);

    // i32 to Number
    let num = Number::from(16);
    let num2 = Number::from(32);

    // Number to big integers
    let big: u64 = num.into();
    let bigger: u128 = num2.into();

    // Rust is smart enough to call `Number::from(<u128>)` when confronted
    // with a request to convert `into()` Number from a u128.
    let signed: u128 = 128;
    let num3: Number = signed.into();   // Note that type-spec is needed here


    // TryFrom and TryInto
    ///////////////////////

    // i32 to EvenNumber. Sometimes works...
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // i32 to EvenNumber, but with `try_into()` instead
    let result: Result<EvenNumber, ()> = 8_i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5_i32.try_into();
    assert_eq!(result, Err(()));


    // To and From Strings
    ///////////////////////

    #[derive(Debug)]
    struct Circle {
        radius: i32
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    impl FromStr for Circle {
        type Err = ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim().parse() {
                Ok(num) => Ok(Circle{ radius: num }),
                Err(e) => Err(e),
            }
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let circle: Circle = " 3 ".parse().expect("Radius should be an integer");
    dbg!(circle);

    let turbo_parsed = "10".parse::<Circle>().unwrap();
    dbg!(turbo_parsed);
}
