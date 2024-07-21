#![allow(unused_variables)]

/**
Rust groups errors into two major categories: recoverable and unrecoverable errors.

Functions return `Result<T, E>` for recoverable errors, and abort program
execution using the `panic!` macro for unrecoverable errors.
*/


fn main() {
    seppuku();
    using_result();
}


/**
Recoverable Errors with Result.

Every function that might fail has to agree to return a `Result` enum:

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

`T` represents the type of the value that will be returned in a success case
within the `Ok` variant, and `E` represents the type of the error that will be
returned in a failure case within the `Err` variant.
*/
fn using_result() {
}


/// Unrecoverable errors
fn seppuku() {
    // You can cause a panic directly:
    //
    // thread 'main' panicked at src/main.rs:21:5:
    // crash and burn
    //~ panic!("crash and burn");

    // Or by running an unrecoverable operation:
    // thread 'main' panicked at src/main.rs:27:6:
    // index out of bounds: the len is 3 but the index is 99
    let v = vec![1, 2, 3];
    //~ v[99];
}
