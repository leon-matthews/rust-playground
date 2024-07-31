#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;


fn main() {
    // Order of definition does not matter
    //~ fizzbuzz_to(20);
    //~ methods();
    closures();
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
    todo!();
}
