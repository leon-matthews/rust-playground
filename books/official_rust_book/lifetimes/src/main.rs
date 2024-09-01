#![allow(dead_code)]
#![allow(unused_variables)]

/*!
Lifetimes a generic that ensure references are valid as long as they need to be.

Dangling references (referencces to values that have left scope) are not
allowed in Rust. The *borrow checker* (the part of the Rust compiler that
compares scopes of value and their references) will not allow a program with
dangling references compile.

Whenever there are two possible interpretations of lifetimes the borrow checker
must be conservative and reject what might be a valid program. Lifetime generics
are used in cases where providing more information to the borrow checker is
necessary.
*/


fn main() {
    let s1 = String::from("long string is long");
    let s2 = String::from("xyz");
    let result = longest(&s1, &s2);
    println!("The longest string is: {result}");
}


/**
Return longest of the given slices
These lifetimes specify the following constraint: the returned reference
will be valid as long as *both* the parameters are valid. ie. The reference
returned will have the shorter of the lifetimes of the two value being
referenced by `a` and `b'.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
