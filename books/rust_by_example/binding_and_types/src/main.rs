
use std::collections::HashMap;

/// Rust by Example, Chapters 4 & 5
/// https://doc.rust-lang.org/rust-by-example/variable_bindings.html
/// https://doc.rust-lang.org/rust-by-example/types.html

fn main() {
    // Casting
    ///////////

    let decimal = 65.4321_f32;
    let integer: u8 = decimal as u8;
    let character = decimal as u8 as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // Saturating cast for floats
    println!("1000.0 as a u8 is : {}", 1000.0 as u8);

    // Literals
    ////////////

    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let j = 1_000_000_000_000_i64;
    let f = 1.0;

    println!("size of `character` in bytes: {}", std::mem::size_of_val(&character));
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `j` in bytes: {}", std::mem::size_of_val(&j));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // A &str is an 8-byte pointer and an 8-byte length
    let story = "Once upon a time...";
    println!("size of `story` in bytes: {}", std::mem::size_of_val(&story));
    println!("value of `story` pointer: {:?}", story.as_ptr());
    println!("value of `story` length: {}", story.len());


    // Inference
    /////////////

    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();

    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    vec.push(elem);
    vec.push('A' as u8);
    dbg!(vec);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    dbg!(map);

    // Aliasing
    ////////////
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;
    type Byte = u8;

    let nanoseconds: NanoSecond = 5;
    dbg!(nanoseconds);
}
