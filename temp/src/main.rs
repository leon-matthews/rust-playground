#![warn(clippy::all)]


fn main() -> effective_limits::Result<()> {
    println!("Effective mem limit: {}", effective_limits::memory_limit()?);
    Ok(())
}
