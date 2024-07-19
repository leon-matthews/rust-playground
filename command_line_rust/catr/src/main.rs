
use anyhow::Result;
use clap::Parser;
use catr::LineNumbers;


#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Number nonempty output lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,

    /// Number all output lines
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// Input file(s)
    #[arg(default_value="-")]
    files: Vec<String>,
}


fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}


pub fn run() -> Result<()> {
    let args = Args::parse();
    let numbering_option = match args {
        Args { number_lines: true, .. } => LineNumbers::ShowAll,
        Args { number_nonblank_lines: true, .. } => LineNumbers::NonBlank,
        _ => LineNumbers::DontShow,
    };

    for filename in &args.files {
        match catr::open(filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => catr::concatenate(file, &numbering_option)?,
        }
    }
    Ok(())
}
