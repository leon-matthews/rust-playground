
use std::fmt;


type Result<T> = std::result::Result<T, DoubleError>;


fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}


#[derive(Debug, Clone)]
struct DoubleError;


impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}


fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


/**
Double the first element in the given vector.

In order:

slice::first()
    Returns the first element of the slice, or None if it is empty.

Option::ok_or()
    Transforms the Option<T> into a Result<T, E>,
    mapping Some(v) to Ok(v) and None to Err(err).

Result::and_then()
    Calls op if the result is Ok, otherwise returns the Err value of self.

Result::map_err()
    Maps a Result<T, E> to Result<T, F> by applying a function to a
    contained Err value, leaving an Ok value untouched.

Result::map()
    Maps a Result<T, E> to Result<U, E> by applying a function to a contained
    Ok value, leaving an Err value untouched.
*/
fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec
        .first()                            // `Option<&&str>`
        .ok_or(DoubleError)                 // `Result<&&str, DoubleError>`
        .and_then(|s| {
            s.parse::<i32>()                // `Result<i32, ParseIntError>`
                .map_err(|_| DoubleError)   // `Result<i32, DoubleError>`
                .map(|i| 2 * i)
        })
}
