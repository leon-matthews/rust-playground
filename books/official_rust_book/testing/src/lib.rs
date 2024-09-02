#![allow(dead_code)]


pub fn greeting(name: &str) -> String {
    format!("Hello {name}")
}


pub struct Guess {
    value: i32,
}


impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got: {}", value);
        }
        Guess { value }
    }
}


#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


#[cfg(test)]
mod tests;