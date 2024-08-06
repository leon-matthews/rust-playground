
use anyhow::Result;

pub mod config;


pub fn run(config: config::Config) -> Result<()> {
    println!("-n {}", config.lines);
    println!("-c {:?}", config.bytes);
    println!("{:#?}", config.files);
    dbg!(&config);
    Ok(())
}
