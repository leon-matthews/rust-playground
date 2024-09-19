/*!
Trying out various ideas from BurntSushi on GitHub for counting word
frequency in Rust.

https://github.com/benhoyt/countwords/
*/
use core::error::Error;
use std::io::{self, BufRead};

// 22% faster than default hasher on 13MB text input
use rustc_hash::{FxHashMap as HashMap};


fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}


/**
Count unique words found in stdin.
*/
fn try_main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdin = io::BufReader::new(stdin.lock());
    let mut counts: HashMap<String, usize> = HashMap::default();

    for result in stdin.lines() {
        let mut line = result?;
        line.make_ascii_lowercase();
        for word in line.split_ascii_whitespace() {
            //~ *counts.entry(String::from(word)).or_insert(0) += 1;
            increment(&mut counts, word);
        }
    }

    let total: usize = counts.values().sum();
    println!("Found {} words, {} unique", total, counts.len());
    Ok(())
}


/**
Increment word's count or create new entry with a count of one.

In this application, avoiding the `entry()` API is faster. We have to perform
two look-ups, but save allocating a new String in the common case where the
word already exists.
*/
fn increment(counts: &mut HashMap<String, usize>, word: &str) {
    // 16% faster than using entry on 13MB text input
    if let Some(count) = counts.get_mut(word) {
        *count += 1;
    } else {
        counts.insert(String::from(word), 1);
    }
}
