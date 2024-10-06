#![allow(dead_code)]

/**
Result is a richer version of the Option type that describes possible error
instead of possible absence.

That is, Result<T, E> could have one of two outcomes:

    Ok(T): An element T was found
    Err(E): An error was found with element E
*/

use std::num::ParseIntError;


fn main() {
    parse_string();
    aliases_for_result();
}


fn parse_string() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}


fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


/// Muliply two numbers given as string slices
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // Nested match statements might be a sign that we've gone in
    // the wrong direction!
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}


/**
Multiply if both values can be parsed from str, otherwise pass on the error.

Use `Result::map()`
*/
fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}


/// Muliply two numbers given as string slices using `?` operator to return error
fn multiply3(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // Bail out with a `Err(ParseIntError)` if parsing fails
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}


// Define a generic alias for a `Result` with the error type `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;


/// Same as `print()`, but we don't need to specify the error type explicity
fn print2(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


fn aliases_for_result() {
    print2(multiply("11", "12"));
    print2(multiply("t", "12"));
}
