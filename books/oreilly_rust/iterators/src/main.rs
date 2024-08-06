#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


/**
Iterator workflow:

1. Create iterator, eg. `from_fn()`
2. Adapt iterater, eg. `filter()`
3. Consume iterator, eg. `count()`, `sum()`
*/
fn main() {
    triangle_example();
    desugar_for_loops();
    iter_vs_into_iter();
    from_fn_example();
}


fn triangle_example() {
    let n = 1_000;
    println!("{}th triangle number is {}.", n, triangle(n));
    println!("{}th triangle number is {}, still.", n, triangle_folded(n));
    assert_eq!(triangle(n), triangle_folded(n));

    /// Calculate the nth triangle number using manual loop
    fn triangle(n: u64) -> u64 {
        let mut sum = 0;
        for i in 1..=n {
            sum += i;
        }
        sum
    }

    /// Like `triangle()`, but using iterator's `fold()`
    fn triangle_folded(n: u64) -> u64 {
        (1..=n).fold(0, |sum, item| sum + item)
    }
}


/**
The `std::iter::Iterator` trait looks like the below.

`Item` is the type the iterator produces. The `next()` method produces
either `Some(v)`, where v in the next value, or `None` to indicate the end.
*/
trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // LOTS of default methods
}


/**
Implementing the `std::iter::IntoIterator` trait makes a type *iterable*.

If there is a natural way to iterate over some type it can implement this trait
which produces an iterator over itself.
*/
trait MyIntoIterator {
    type Item;                          // The value the iterator produces
    type IntoIter: Iterator;            // The iterator itself
    fn into_iter(self) -> Self::IntoIter;
}


/**
For loop creates an iterator, then repeatedly calls `next()` on it.

Once it returns `None` rather than `Some` the for loop ends.
*/
fn desugar_for_loops() {
    let v = vec!["antimony", "arsenic", "aluminium", "selenium"];

    // For loops are easy!
    for element in &v {
        print!("{element} ");
    }

    // Or are they...?
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        print!("{element} ");
    }

    println!();
}


/**
Collections also implement `iter()` and `iter_mut()`.

Why? Convenience. Compare the syntax below. These are the most common way to
get natural iterators over a type. The type's implement these in whatever way
they like.

- Iterator from `into_iter()` may yield any of `T`, `&T`, or `&mut T`,
  depending on the context.
- Iterator from `iter()` will yield `&T`, by convention.
- Iterator from `iter_mut()` will yield `&mut T`, by convention.

*/
fn iter_vs_into_iter() {
    // Calling `into_iter()` directly is tricky, because the dot-operator
    // binds *before* the reference.
    let mut v = vec![String::from("Leon"), String::from("Matthews")];

    // Use `into_iter()`, careful about precedence
    let it = (&mut v).into_iter();      // std::slice::IterMut<'_, String>
    let it = (&v).into_iter();          // std::slice::Iter<'_, String>
    let mut it = v.into_iter();         // std::vec::IntoIter<String>

    assert_eq!(it.next(), Some("Leon".to_string()));
    assert_eq!(it.next(), Some("Matthews".to_string()));
    assert_eq!(it.next(), None);

    // Easier syntax
    let mut v2 = vec![String::from("Leon"), String::from("Matthews")];

    // `iter()`
    // Same as: `(&v).into_iter()`
    let it2 = v2.iter();                // std::slice::Iter<'_, String>

    // `iter_mut()`
    // Same as: `(&mut v).into_iter()`
    let it3 = v2.iter_mut();            // std::slice::IterMut<'_, String>
}


/**
Given a function that returns `Option<t>`, `std::iter::from_fn` returns an
iterator than simply calls the function to produce its items.
*/

use std::iter::from_fn;

fn from_fn_example() {
    // Plain function
    fn answer() -> Option<i32> { Some(42) }
    let answers = from_fn(answer);
    for i in answers {
        println!("{i}");
        break;
    }

    // Or just a closure
    let answers = from_fn(|| Some(42));
    for i in answers {
        println!("{i}");
        break;
    }

    // Consume!
    let answers = from_fn(|| Some(42));
    let collected: Vec<u8> = answers.take(10).collect();
    println!("Answers: {:?}", collected);
}
