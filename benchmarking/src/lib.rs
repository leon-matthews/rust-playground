
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


/// Read lines into vector, manually pushing each line
pub fn read_lines_push(path: &Path, num_lines: Option<usize>) -> io::Result<Vec<String>> {
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut lines = Vec::<String>::new();
    if let Some(num_lines) = num_lines{
        lines.reserve(num_lines);
    }
    for result in reader.lines() {
        lines.push(result?);
    }
    Ok(lines)
}


/// Read lines into vector, using `collect()`
pub fn read_lines_collect(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<io::Result<Vec<_>>>()?;
    Ok(lines)
}
