
fn main() {
    // Collect into a result
    let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];
    let result: Vec<_> = results.iter().map(|n| n).collect();
    dbg!(&result);
}
