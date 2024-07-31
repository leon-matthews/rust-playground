#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use std::fmt;


fn main() {
    basics();
    operators();
    tuples();
    arrays();
    slices();
}


fn basics() {
    // Type annotation usually optional, shadowing is allowed.
    let logical = true;
    let logical: bool = true;

    // Regular vs suffix annotation
    let gst: f64 = 12.5;
    let gst = 15.0_f64;         // (Underscores in numeric literals are ignored)

    // Default types used
    let default_integer = 7;    // `i32`
    let default_float = 3.0;    // `f64`

    // Type inference is more powerful than I expected!
    let mut not_i32 = 14;
    not_i32 = 123456789_u64;

    // Only mutable variables can be changed (or reassigned)
    let mut mutable = 13;
    mutable = 15;
    mutable += 1;

    // But their type can't be changed (except by shadowing)
    // error[E0308]: mismatched types expected integer, found `bool`
    //~ mutable = false;
    let mutable = false;
}


fn operators() {
    // Integer addition and subtraction
    println!("1 + 2 = {}", 1_u32 + 2);
    println!("1 - 2 = {}", 1_i32 - 2);  // Overflow panic if first type is `u32`

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operators
    println!("0011 AND 0101 is {:04b}", 0b0011_u32 & 0b0101);
    println!("0011 OR  0101 is {:04b}", 0b0011_u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011_u32 ^ 0b0101);
    println!("1 << 5 is {0} ({0:b})", 0x01 << 6);
    println!("{0:#x} ({0}) >> 2 is {1:#x} ({1})", 0x80u32, 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}


/// Matrix tuple struct for `tuples()` examples
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);


impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

impl Matrix {
    /// Build new matrix from current
    fn transpose(&self) -> Matrix {
        Matrix(self.0, self.2, self.1, self.3)
    }
}


/// Take a tuple and return it, reversing its order.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    (pair.1, pair.0)
}


fn tuples() {
    let long = (
        false, true,
        1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2_f64,
    );

    // Tuples of up to 12 elements can be printed
    println!("{:?}", long);
    println!("First element is {}", long.0);

    // Nesting is fine
    let nested = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("{:?}", nested);

    // That reverse function is very elegant
    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("Reversed pair is {:?}", reverse(pair));

    // Tuple deconstruction
    let tuple = (1, "hello", 4.5, true);
    println!("Tuple is {:?}", tuple);
    let (a, b, c, d) = tuple;
    println!("Deconstructed: {} {} {} {}", a, b, c, d);


    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", matrix.transpose());
}


fn arrays() {
    // Fixed-size array, types can often be infered
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let xs = [1, 2, 3, 4, 5];

    // Indexing, but will panic if out of bounds
    println!("Array {:?}, first is {}, last is {}", xs, xs[0], xs[xs.len() - 1]);

    // Safe indexing using `get()`
    if let Some(elem) = xs.get(2_000) {
        println!("How is this possible: {}", elem);
    } else {
        println!("Nothing found at that silly index");
    }

    // Initialise array of some length with same value
    let ys = [0_u64; 5_000];
    println!(
        "Array length = {}, first is {}, last is {}",
        ys.len(), ys[0], ys[ys.len() - 1]
    );

    // Allocated on the stack
    println!("Array uses {} bytes", size_of_val(&ys));
}


fn analyze_slice(slice: &[u32]) {
    println!("The slice has {} elements, first element is {}",
        slice.len(), slice[0]);
}


fn slices() {
    let xs = [1_u32, 2, 3, 4, 5];

    // Array can be borrowed as slice
    analyze_slice(&xs);
    analyze_slice(&xs[1..4]);
    analyze_slice(&xs[2..3]);
}
