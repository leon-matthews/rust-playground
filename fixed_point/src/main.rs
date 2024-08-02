
use fixed_point::Q7;


fn main() {
    let half = Q7::from(0.5);
    println!("Half is {:?}", half);
    println!("Half back to f64 is {:?}", f64::from(half));

    let minus_half = Q7::from(-0.5);
    println!("Minus half is {:?}", minus_half);
    println!("Minus half back to f64 is {:?}", f64::from(minus_half));

    println!("Smallest difference is {}", Q7::EPSILON);
}
