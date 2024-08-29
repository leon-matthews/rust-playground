#![allow(dead_code)]


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}


fn main() {
    let point = Point { x: 1000, y: 729 };
    let r = &point;
    let rr = &r;
    println!("{r:?}");
}
