#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::{self, BufRead};


/// TODO: Compare reading lines from buffered reader, collect vs manual loop
fn grep<R>(target: &str, reader: R) -> io::Result<()> where R: BufRead {
    // Collect lines into vector
    //~ let mut lines = vec![];
    //~ for result in reader.lines() {
        //~ lines.push(result?);
    //~ }

    // Collect lines into vector using `collect()`
    let lines = reader.lines().collect::<io::Result<Vec<_>>>()?;
    dbg!(&lines);
    Ok(())
}



/// A simple brute-force solution is to iterate on every number in the range
/// and determine if it's divisible by either 3 or 5. This solution will give us
/// a good baseline to compare the other solutions to.
pub fn euler1_simple(input: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..input {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i as i64;
        }
    }
    sum
}


/// Arithmetic series solution
pub fn euler1_series(input: i64) -> i64 {
    let val = input - 1;
    let n_3 = val / 3;
    let n_5 = val / 5;
    let n_15 = val / 15;

    let sum_three = 3 * n_3 * (1 + n_3) / 2;
    let sum_five = 5 * n_5 * (1 + n_5) / 2;
    let sum_fifteen = 15 * n_15 * (1 + n_15) / 2;

    sum_three + sum_five - sum_fifteen
}
