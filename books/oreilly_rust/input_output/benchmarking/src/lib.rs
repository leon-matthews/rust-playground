

/// TODO: Compare reading lines from buffered reader, collect vs manual loop
fn grep2<R>(target: &str, reader: R) -> io::Result<()> where R: BufRead {
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
