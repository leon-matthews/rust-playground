#![allow(unused_variables)]


fn main() {
    let (four, nine) = (4, 9);
    explicit_lifetimes(&four, &nine);

    // Functions
    let mut one = 1;
    print_one(&one);
    add_one(&mut one);
    print_one(&one);
    println!("{}", pass_x(&four, &nine));

    // Elision
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}


// Explicit lifetime annotations
fn explicit_lifetimes<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}


/**
Functions

Unless elided, function signatures containing references *must* annotate
lifetimes, and the return type's lifetime must by the same as an input or be
static.
*/

// One input reference with lifetime which must live as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}


/**
Elision

Some lifetime patterns are overwhelmingly common and so the borrow checker will
allow you to omit them to save typing and to improve readability.

1. The compiler assigns a lifetime parameter to each reference parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned
   to all output lifetime parameters.

3. If there are multiple input lifetime parameters, but one of them is &self,
   the lifetime of self is assigned to all output lifetime parameters.
*/
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

// Like `elided_input()`, but with explicit lifetimes
fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}


fn elided_pass(x: &i32) -> &i32 {
    x
}

// Like `elided_pass()`, but with explicit lifetimes
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}
