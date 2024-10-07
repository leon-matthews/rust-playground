#![allow(dead_code)]
#![allow(unused_variables)]

use std::num::ParseIntError;


fn main() {
    let easy = vec!["42", "93", "18"];
    let empty = vec![];
    let invalid = vec!["tofu", "93", "18"];

    // Unwrap and panic!
    println!("The first doubled is {}", double_first(easy.clone()));

    // Wrap the result inside the option
    println!("Result inside option (easy): {:?}",       // Some(Ok(84))
        result_inside_option(easy.clone()));
    println!("Result inside option (empty): {:?}",      // None
        result_inside_option(empty.clone()));
    println!("Result inside option (invalid): {:?}",    // Some(Err(ParseIntError { kind: InvalidDigit }))
        result_inside_option(invalid.clone()));

    // Wrap option inside result
    println!("Result inside option (easy): {:?}",       // Ok(Some(84))
        option_inside_result(easy.clone()));
    println!("Result inside option (empty): {:?}",      // Ok(None)
        option_inside_result(empty.clone()));
    println!("Result inside option (invalid): {:?}",    // Err(ParseIntError { kind: InvalidDigit })
        option_inside_result(invalid.clone()));
}


/**
Double the first number in the given vector of strings.

Two things can go wrong in our function:

1. `Vec::first()` returns an Option, None if the vector is empty.
2. `parse::<i32>` returns a `Result<i32, ParseIntError>`.

This version of the function will painic if either thing happens.
*/
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();   // `Option::None` if vector empty
    2 * first.parse::<i32>().unwrap()   // `Result::Err(ParseIntError)` if not number
}


/**
Combine our errors by putting the result inside the option using map.
*/
fn result_inside_option(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}


/**
There are times when we'll want to stop processing on errors (like with ?) but
keep going when the Option is None. The transpose function comes in handy to
swap the Result and Option.
*/
fn option_inside_result(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let option = result_inside_option(vec);
    option.transpose()
}
