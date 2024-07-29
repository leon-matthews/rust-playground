#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::{self, stdout, Write};


/**
Traits are like interfaces or abstract base classes from other languages.

Part of the `std::io::Write` trait looks like this.
*/
trait SampleWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn flush(&mut self) -> io::Result<()>;
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Trait objects vs generic functions specify trait bounds
    // Various ways of specifiying trait bound function arguments.
    {
        let mut cout = stdout().lock();
        trait_bound_1(&mut cout)?;
        trait_bound_2(&mut cout)?;
        trait_bound_3(&mut cout)?;
        // Release stdout lock
    }
    trait_objects();

    // TODO: Custom traits
    // Page 266 in Chapter 11

    Ok(())
}





fn trait_objects() {
    let mut buf: Vec<u8> = vec![];

    // Error: doesn't have a size known at compile time
    // The struct that implements `Write` could be any size!
    //~ let writer: dyn Write = buf;

    // We have to explicitly take use an 'mute ref' instead
    // These are called 'trait objects', they are 'fat' pointers using a vref table.
    // Like any other reference type, they can be `mut` or shared.
    let writer: &mut dyn Write = &mut buf;
}


/**
Use a writer without caring about its type.

Note that to be usable a trait has to be in scope, otherwise its methods
are hidden.

Here `out` is a mutable reference to any value that implements the `Write` trait.
*/
fn trait_bound_1(out: &mut dyn Write) -> io::Result<()> {
    out.write(b"Hello\n").unwrap();
    out.flush().unwrap();
    Ok(())
}

/**
Trait bound specified in generic type signature.

Specifying traits this way does not require trait objects' fat pointers
or their `vtable`, but does result in mulitple copies of the function being
created, one for each type used, via the 'monomorphisation' process.

Which type `T` stands for depends on how function is used:

    trait_bound_2(&mut local_file)?;    // Calls trait_bound_2::<File>
    trait_bound_2(&mut bytes)?;         // Calls trait_bound_2::<Vec<u8>>

*/
fn trait_bound_2<T: Write>(out: &mut T) -> io::Result<()> {
    out.write(b"Hello 2\n").unwrap();
    out.flush().unwrap();
    Ok(())
}


/// Trait bound using `where` syntax, but still generic.
fn trait_bound_3<T>(out: &mut T) -> io::Result<()>
where T: Write {
    out.write(b"Hello 3\n").unwrap();
    out.flush().unwrap();
    Ok(())
}
