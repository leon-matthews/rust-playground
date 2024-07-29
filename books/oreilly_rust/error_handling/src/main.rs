#![allow(dead_code)]
#![allow(unused_variables)]

/// Programming Rust (O'Reilly, 2nd Edition)
/// Chapter 7: Error Handling


#[derive(Debug, Clone)]
pub struct GenericError {
    pub message: String,
}


impl fmt::Display for GenericError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.message)
    }
}

/// Use default definition for `io::Error` trait.
impl Error for GenericError {}


/// Error handling is all about `panic` or `Result`
fn main() {
    panic!("Hello, world!");
}
