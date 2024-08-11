#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;


fn main() {
    fizzbuzz_to(20);
    methods();
    closures();
    capturing_immutable();
    capturing_mutable();
    capturing_move();
    functions_taking_closures();
    function_returning_closures();
    std_find();
    higher_order_functions();
    diverging_functions();
}


/// Print correct 'fizzbuzz' response for the given integer.
fn fizzbuzz(number: i32) {
    if is_divisible_by(number, 15) {
        println!("Fizz Buzz");
    }
    else if is_divisible_by(number, 5) {
        println!("Buzz");
    }
    else if is_divisible_by(number, 3) {
        println!("Fizz");
    }
    else {
        println!("{number}");
    }
}


/// Run `fizzbuzz` on every positive integer up to `limit`, inclusive.
fn fizzbuzz_to(limit: i32) {
    for n in 1..=limit {
        fizzbuzz(n);
    }
}


/// Can the first number be divided cleanly by the second?
fn is_divisible_by(first: i32, second: i32) -> bool {
    // Avoid divide-by-zero
    if second == 0 {
        return false;
    }

    // Elegant!
    first % second == 0
}


#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}


/// Methods and 'associated functions' go into `impl` block
impl Point {
    /// Associated functions don't need to be called with an instance.
    /// These functions are generally used like constructors.
    fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Another associated function
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

}


struct Rectangle {
    p1: Point,
    p2: Point,
}


impl Rectangle {
    /// Method uses `&self` as syntax-sugar for `self: &Self`
    fn area(&self) -> f64 {
        // Destructure
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // Calculate area
        ((x1 - x2) * (y1 - y2)).abs()
    }

    /// Another method
    fn perimeter(&self) -> f64 {
        // Destructure
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    /// This method requires the caller object to be mutable
    /// `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}


impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) to ({}, {})",
            self.p1.x, self.p1.y, self.p2.x, self.p2.y)
    }
}


fn methods() {
    let origin = Point::origin();
    println!("{:?} is the origin", origin);

    let point = Point::new(3.0, 4.0);
    println!("{:?} is another point", point);

    let rectangle = Rectangle {
        p1: origin,
        p2: point,
    };

    println!("Rectangle {} has area {}, and perimeter of {}",
        rectangle, rectangle.area(), rectangle.perimeter());

    let mut rectangle = Rectangle {
        p1: Point::new(3.0, 4.0),
        p2: Point::origin(),
    };
    rectangle.translate(10.0, 10.0);
    println!("Rectangle after translate {}", rectangle);
}


fn closures() {
    let outer_var = 42;

    // Functions can't refer to variables in the enclosing environment
    // error[E0434]: can't capture dynamic environment in a fn item
    //~ fn add_one(i: i32) -> i32 {
        //~ i + outer_var
    //~ }

    // Closures don't need type annotation
    let annotated = |i: i32| -> i32 { i + outer_var };
    let inferred  = |i| i + outer_var;

    println!("`outer_var` plus three is {}", annotated(3));
    println!("`outer_var` plus three is still {}", inferred(3));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}


/**
Closures can capture outside variables

Either:
    By reference: `&T`,
    by mutable reference: `&mut T`, or
    by value `T`.
*/
fn capturing_immutable() {
    let colour = String::from("green");

    // Print macro only need immutable reference, so that's what it gets!
    let print = || println!("Variable `colour` is {:?}", colour);

    // Call closure using the borrow
    print();

    // Closure only holds an immutable reference, so we can reborrow
    let _reborrow = &colour;
    print();

    // Move works, only after last use of closure
    let _colour_moved = colour;
    // error[E0505]: cannot move out of `colour` because it is borrowed
    //~ print();
}


/**
A closure to increment our `count` could take either `&mut count` or `count`
but `&mut count` is less restrictive so it takes that.
*/
fn capturing_mutable() {
    let mut count = 0;

    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("Variable `count` is {}", count);
    };

    inc();
    inc();

    // We can borrow once we've finished using the closure.
    let _reborrow = &count;
    let _reborrow = &mut count;
    //~ inc();
}


fn capturing_move() {
    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value - and it does.
    let consume = || {
        println!("`movable`: {:?}", movable);
        drop(movable);
    };

    // Can only be called once
    consume();
    // error[E0382]: use of moved value: `consume`
    //~ consume();

    // Move can be forced using `move`
    let haystack = vec![2, 3, 5, 7];
    let is_prime = move |needle| haystack.contains(needle);

    // Note that vec::contains() requires &T, and rust infered that to be the
    // type of the closure's argument.
    println!("One is prime: {}", is_prime(&1));

    // Can can call the closure multiple times
    println!("Seven is prime: {}", is_prime(&7));
    println!("Eleven is prime: {}", is_prime(&11));

    // We didn't *need* to move haystack. Removing `move` will allow this line.
    // error[E0382]: borrow of moved value: `haystack`
    //~ println!("There are {} primes in existence", haystack.len());
}


/// Writing functions that take closures as arguments.
/// Generics are required because each closure is its own type - a new
/// anonymous structure is created to hold captured variables then a marker
/// trait is applied.
fn functions_taking_closures() {
    // Take closure and call it
    fn apply<F>(f: F) -> () where F: FnOnce() {
        f()
    }

    // Put a little sugar on it (note return syntax)
    fn apply2(f: impl FnOnce() -> ()) {
        f()
    }

    apply(|| println!("Hello, world") );
    apply2(|| println!("Hello, world") );
}


/// Possible, but requires using `impl` as type is anonymous
fn function_returning_closures() {
    // Fn
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_string();
        move || println!("This is a {}", text)
    }

    let fn_plain = create_fn();
    fn_plain();
    create_fn()();

    // FnMut
    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_string();
        move || println!("This is a {}", text)
    }

    let mut fn_mut = create_fnmut();
    fn_mut();
}

/**
Iterator::find is a function which iterates over an iterator and searches for
the first value which satisfies some condition. If none of the values satisfy
the condition, it returns None.
*/
fn std_find() {
    // find()
    // Provides reference to item as closure's argument
    let v = vec![2, 3, 5, 7, 11, 13, 17, 19];
    println!("Find 11: {:?}", v.iter().find(|&x| *x == 11));
    println!("Find 111: {:?}", v.iter().find(|&x| *x == 111));

    // position()
    // Provides
    println!("Position of 11: {:?}", v.iter().position(|&x| x == 11));
    println!("Position of 111: {:?}", v.iter().position(|&x| x == 111));
}


/// Functions that take one or more functions and/or produce a more useful function
fn higher_order_functions() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the numbers with odd squares under 1000");
    let upper = 1_000;
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)            // ALL natural numbers squared
        .take_while(|&n| n < upper)     // Stop when squares exceed limit
        .filter(|&n| is_odd(n))         // Pass through only odd numbers
        .sum();                         // Accumulate the sum

    dbg!(&sum_of_squared_odd_numbers);
}


/// Diverging functions never return.
/// They are marked using !, which is an empty type.
fn diverging_functions() {
    fn foo() {
        panic!("This call never returns.");
    }

    println!("Hello");
}
