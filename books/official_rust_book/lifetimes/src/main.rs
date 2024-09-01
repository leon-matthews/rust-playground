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

The Rust compiler infers lifetimes following these rules:

1. Each reference input parameter gets its own lifetime, ie. `'a`, then `'b`, etc.
2. If there is only one input lifetime parameter, that is assigned as the
   lifetime to all of the output parameters.
3. If one of the input lifetime parameters is to `self`, the function is a
   method and the lifetime of `self` is used for all output parameters.

These rules mean that lifetimes can often be left off function definitions,
a process refereed to as 'lifetime elision'.
*/


fn main() {
    // Lifetimes in function signatures
    let s1 = String::from("long string is long");
    let s2 = String::from("xyz");
    let result = longest(&s1, &s2);
    println!("The longest string is: {result}");

    // Lifetimes in struct definitions
    let novel = String::from(
        "After warming up, he ran around the park. Down the road. Over the bridge."
    );

    let first_sentence = novel
        .split_inclusive(['.', '!', '?'])
        .next()
        .expect("Could not find a full-stop");

    // Break nicely into sentences, just for fun.
    println!(
        "{:#?}",
        novel
            .split_inclusive(['.', '!', '?'])
            .map(|line| line.trim_start())
            .collect::<Vec<_>>()
    );

    let e = Excerpt {
        part: first_sentence,
    };

    println!("First sentence {}", e.part);
    println!("First word: {:?}", first_word(&novel));
    println!("Only word: {:?}", first_word("Leon"));
    println!("Empty word: {:?}", first_word(""));
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


/**
Structs containing references *require* a named lifetime parameter.
*/
struct Excerpt<'a> {
    part: &'a str,
}


/**
This function doesn't need explicit lifetimes thanks to 'lifetime elision'.
*/
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        None => s,
        Some(i) => &s[..i]
    }
}
