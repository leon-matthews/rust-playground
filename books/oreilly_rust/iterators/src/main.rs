#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};
use std::iter::{from_fn, repeat, successors};
use std::str::FromStr;


/**
Iterator workflow:

1. Create iterator, eg. `from_fn`, `repeat`, `successors`
2. Adapt iterater, eg. `filter`, 'take'
3. Consume iterator, eg. `count`, `product`, `sum`
*/
fn main() {
    triangle_example();
    desugar_for_loops();
    iter_vs_into_iter();
    from_fn_example();
    successors_example();
    map_and_filter();
    filter_map_adapter();
    flat_map_adapter();
    flatten_adapter();
    zip_example();
    by_ref_example();
    fold_example();
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
    let _it = (&mut v).into_iter();     // std::slice::IterMut<'_, String>
    let _it = (&v).into_iter();         // std::slice::Iter<'_, String>
    let mut it = v.into_iter();         // std::vec::IntoIter<String>

    assert_eq!(it.next(), Some("Leon".to_string()));
    assert_eq!(it.next(), Some("Matthews".to_string()));
    assert_eq!(it.next(), None);

    // Easier syntax
    let mut v2 = vec![String::from("Leon"), String::from("Matthews")];

    // `iter()`
    // Same as: `(&v).into_iter()`
    let _it2 = v2.iter();               // std::slice::Iter<'_, String>

    // `iter_mut()`
    // Same as: `(&mut v).into_iter()`
    let _it3 = v2.iter_mut();           // std::slice::IterMut<'_, String>
}


/**
Given a function that returns `Option<t>`, `std::iter::from_fn` returns an
iterator than simply calls the function to produce its items.
*/
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


/// Iterator source where next item is computed based on the preceding one.
fn successors_example() {
    let powers_of_two = successors(Some(2_u16), |n| n.checked_mul(2))
        .collect::<Vec<_>>();
    println!("{:?}", powers_of_two);
}


/// Iterator adapters produce another iterator from an existing one
/// `map` applies closure to each item
/// `filter` skips items when its closure returns false
fn map_and_filter() {
    let text = "  ponies \n giraffes\niguanas  \nsquid".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    println!("{:?}", v);
}


/**
`filter_map` is like `map`, but allows us to either transform or drop.
Specifically, it yields only the values for which the supplied closure
returns `Some(value)`.
*/
fn filter_map_adapter() {

    let text = "1\nfrond .25  289\n3.14415 estuary\n";
    for number in text
        // Break into 'tokens'
        .split_whitespace()
        // Convert to f64, return result, convert to option,
        // then only pass on values that are wrapped in `Some`
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{number}");
    }
}


/**
Creates an iterator that works like map, but flattens nested structure.
*/
fn flat_map_adapter() {
    let mut cities = HashMap::new();
    cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    cities.insert("USA", vec!["Portland", "Nashville", "New York"]);
    cities.insert("Brazil", vec!["Brasilia", "Sao Paulo"]);
    cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    cities.insert("Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya", "USA"];

    // If closure produces iterator, consume that too
    for city in countries.iter().flat_map(|country| &cities[country]) {
        print!("{city} ");
    }
    println!();
}


/**
Concatinates iterators items, assumingc each item is itself an iterable.
*/
fn flatten_adapter() {
    // Map cities to their parks
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    // Build vector of all parks
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();
    println!("{:?}", all_parks);

    // Flatten can take a vector of options and keep only the values in `Some`
    // `Option` implements `IntoIterator` where it is represented as a
    // sequence of zero or one value. The same trick works with `Result`, where
    // errors are then discarded.
    let options = vec![None, Some("Day"), None, Some("One")];
    let values: Vec<_> = options.into_iter().flatten().collect();
    assert_eq!(values, ["Day", "One"]);
}


/// Produce tuples until one iterator runs out
fn zip_example() {
    let going = repeat("going");
    let endings = ["once", "twice", "chicken soup with rice"];
    for (first, second) in going.zip(endings) {
        println!("{} {}", first, second);
    }
}


/**
Allow iterator to be detached from adapter, by temporarily taking control of
it by reference.
*/
fn by_ref_example() {
    let email =
        "To: jimb\n\
        From: id\n\
        \n
        Donuts in break room!\n";
    let mut lines = email.lines().map(|line| line.trim());

    // Take temporary control of `lines` iterator using `by_ref()`
    println!("Headers:");
    for header in lines.by_ref().take_while(|line| !line.is_empty() ) {
        println!("  {header}");
    }

    // Half-used `lines` still has some content left
    println!("Body:");
    for body in lines {
        if body.is_empty() {
            continue;
        }
        println!("  {body}");
    }
}

/**
Folds every element into an accumulator by applying an operation, returning
the final result.

The iterator consumer's first argument is the initial value of the
accumulator, the second is a closure. The closure takes two arguments: the
accumulator's current value then the current item.

The returned result is the final value of the accumulator.
*/
fn fold_example() {
    let a = [5, 6, 7, 8, 9, 10];

    // Count implemented by just adding one to accumulator
    assert_eq!(a.iter().fold(0, |accumulator, _| accumulator + 1), 6);

    // Sum by adding value to accumulator
    assert_eq!(a.iter().fold(0, |accumulator, n| accumulator + n), 45);

    // Product by multiplying, starting with one
    assert_eq!(a.iter().fold(1, |accumulator, n| accumulator * n), 151_200);

    // Max manually
    assert_eq!(a.iter().fold(i32::MIN, |acc, n| if *n > acc { *n } else { acc } ), 10);

    // Max using stdlib `max()` works, but function needs clones, not references
    assert_eq!(a.iter().cloned().fold(i32::MIN, std::cmp::max ), 10);
}
