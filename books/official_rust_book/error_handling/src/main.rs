#![allow(unused_imports)]
#![allow(unused_variables)]

use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::panic;

/**
Rust groups errors into two major categories: recoverable and unrecoverable errors.

Functions return `Result<T, E>` for recoverable errors, and abort program
execution using the `panic!` macro for unrecoverable errors.
*/
fn main() -> Result<(), Box<dyn Error>> {
    seppuku();
    open_or_create_with_match();
    open_or_create_using_closures();

    // Catch panics...
    let filename = "no-such-file.txt";

    // ...unwrap
    let result = panic::catch_unwind(|| {
        unwrap_panic(&filename);
    });
    assert!(result.is_err());

    // ...expect
    let result = panic::catch_unwind(|| {
        expect_panic(&filename);
    });
    assert!(result.is_err());

    // Propagating errors
    let filename = "username.txt";
    let username = read_username_from_file(&filename)
        .expect("No username file found");
    println!("Username: {username}");

    let username = read_username_from_file2(&filename)
        .expect("No username file found");
    println!("Username: {username}");

    let username = read_username_from_file3(&filename)
        .expect("No username file found");
    println!("Username: {username}");

    // When a main function returns a Result<(), E>, the executable will exit
    // with a value of 0 if main returns Ok(()) and will exit with a nonzero
    // value if main returns an Err value.
    println!("Username: {}", fs::read_to_string("no-such-file")?);
    Ok(())
}


/// Propagate errors manually
fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let result = File::open(filename);
    let mut file = match result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username.trim().to_string()),
        Err(e) => return Err(e),
    }
}


/// Propagate errors using `?` shortcut operator (`ErrorPropagationExpression`)
fn read_username_from_file2(filename: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username.trim().to_string())
}


/// Propagate errors using nice shortcut function
fn read_username_from_file3(filename: &str) -> Result<String, io::Error> {
    let username = fs::read_to_string(filename)?;
    Ok(username.trim().to_string())
}


/**
We know that result can NEVER fail, so just unwrap it!

    thread 'main' panicked at src/main.rs:25:46:
    called `Result::unwrap()` on an `Err` value: Os {
        code: 2, kind: NotFound,
        message: "No such file or directory",
    }

*/
fn unwrap_panic(filename: &str) {
    let greeting_file = File::open(filename).unwrap();
}


/**
Let's add a message just in case the impossible happens.

    thread 'main' panicked at src/main.rs:33:10:
    Distribution should contain greeting file: Os {
        code: 2, kind: NotFound,
        message: "No such file or directory",
    }
*/
fn expect_panic(filename: &str) {
        let greeting_file = File::open(filename)
    .expect("Distribution should contain greeting file");
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
fn open_or_create_with_match() {
    let filename = "hello-match.txt";
    let greeting_file_result = File::open(filename);
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(created_file) => created_file,
                Err(error) => panic!("Problem creating the file: {error:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            },
        }
    };
}


/// Open-or-create logic, but slightly neater using closures and `unwrap_or_else()`
fn open_or_create_using_closures() {
    let filename = "hello-closures.txt";
    let greeting_file = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
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
