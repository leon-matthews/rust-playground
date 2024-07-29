#![allow(dead_code)]

use std::io::Result;


fn main() {
    println!("Hello, world!");
}

/**
Traits are like interfaces or abstract base classes from other languages.

Part of the `std::io::Write` trait looks like this.
*/
trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}


/**
Use a writer without caring about its type.

Here `out` is a mutable reference to any value that implements the `Write` trait.
*/
fn say_hello(out: &mut dyn Write) -> Result<()> {
    out.write(b"Hello").unwrap();
    out.flush().unwrap();
    Ok(())
}
