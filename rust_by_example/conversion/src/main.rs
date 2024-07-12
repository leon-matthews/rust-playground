#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

/// Conversion
/// Rust by Example, Chapter 6
/// https://doc.rust-lang.org/rust-by-example/conversion.html


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


fn main() {
    // str to String
    let my_str = "hello";
    let my_string = String::from(my_str);

    // i32 to Number
    let num = Number::from(16);
    let num2 = Number::from(32);
    println!("My number is {:?}", num);

    // Number to big integers
    let big: u64 = num.into();
    let bigger: u128 = num2.into();

    // Rust is smart enough to call `Number::from(<u128>)` when confronted
    // with a request to convert `into()` Number from a u128.
    let signed: u128 = 128;
    let num3: Number = signed.into();   // Note that type-spec is needed here
}
