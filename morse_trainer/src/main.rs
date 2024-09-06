#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use morse_trainer::Morse::{self, *};


fn main() {
    let bytes = vec![b's', b't', b'e', b'l', b'l', b'a'];
    let codes: Vec<Morse> = bytes.iter()
        .map(|byte| match byte {
            b'a' => [Dit, Dah],
            b'b' => [Dah, Dit, Dit, Dit],
            b'c' => [Dah, Dit, Dah, Dit],
            b'd' => [Dah, Dit, Dit],
            b'e' => [Dit],
            b'f' => [Dit, Dit, Dah, Dit],
            _ => [],
        })
        .flatten()
        .collect();
    println!("{codes:?}");
}
