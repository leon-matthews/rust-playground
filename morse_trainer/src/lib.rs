#![allow(dead_code)]

/**
The various possibles things that a Morse transmission can contain.
*/
#[derive(Debug)]
pub enum Morse {
    Dit,            // 1 time unit on
    Dah,            // 3 dits on
    InterGap,       // 1 dit off between elements
    ShortGap,       // 3 dits off between letters
    MediumGap,      // 7 dits off between words
}

/**
Return true if given byte is an ascii character transmitable in Morse Code.

Strictly International Morse code (Recommendation ITU-R M.1677-1)
*/
fn is_morseable(byte: u8) -> bool {
    match byte {
        32 => true,                     // Space
        34 => true,                     // "
        39..=41 => true,                // ' ( )
        43..=47 => true,                // + , - . /
        48..=57 => true,                // Numbers
        58 => true,                     // :
        61 => true,                     // =
        63..=64 => true,                // ? @
        97..=122 => true,               // Lower-case letters
        _ => false,
    }
}


/**
String containing only characters that can be sent via MorseCode.
*/
fn to_morse(string: &str) -> String {
    let lower = string.trim().to_ascii_lowercase();
    let bytes: Vec<u8> = lower
        .bytes()
        // Drop non-morse and non-ASCII characters
        .filter(|b| is_morseable(*b))
        .scan(0, |last, current| {
            // Drop repeated spaces
            if *last == 32 && current == 32 {
                Some(None)
            } else {
                *last = current;
                Some(Some(current))
            }
        })
        .flatten()
        .collect();
    String::from_utf8(bytes).expect("Illegal characters found")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_morse_pangram() {
        let pangram = to_morse("Sphinx of black quartz judge my vow");
        assert_eq!(pangram, "sphinx of black quartz judge my vow");
    }

    #[test]
    fn to_morse_numbers() {
        let numbers = to_morse("1234567890");
        assert_eq!(numbers, "1234567890");
    }

    #[test]
    fn to_morse_punctuation() {
        // All ASCII punctuation
        let punctuation = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

        // Only strict international Morse code punctuation
        assert_eq!(to_morse(punctuation), "\"'()+,-./:=?@");
    }

    #[test]
    fn to_morse_emoji() {
        // Drop emoji AND the excess spaces around it
        let greeting = " Rusty 🦀 Crab ";
        assert_eq!(to_morse(greeting), "rusty crab");
    }
}
