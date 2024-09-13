#![allow(dead_code)]


#[derive(Debug, Default)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}


fn main() {
    // Full default
    let c = Color::default();
    println!("{:?}", c);

    // Implicit default
    let c: Color = Default::default();
    println!("{:?}", c);

    // Default using struct update syntax
    let c2 = Color {
        blue: 255,
        ..Color::default()
    };
    println!("{:?}", c2);

    // Implicit using struct update syntax
    let c3 = Color {
        red: 255,
        ..Default::default()
    };
    println!("{:?}", c3);
}
