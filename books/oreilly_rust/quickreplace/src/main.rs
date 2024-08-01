#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::env;
use std::fs;

use text_colorizer::*;


fn main() {
    // Parse arguments
    let args = parse_args();
    dbg!(&args);

    // Read input
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                "Error:".red().bold(), args.filename, e);
            std::process::exit(2);
        }
    };

    // Write output
    match fs::write(&args.output, &data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                "Error:".red().bold(), args.filename, e);
            std::process::exit(3);
        }
    }
}


#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}


/// Extract command-line options
/// Will print usage message and exit program if error found.
fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} expected 4 arguments, got {}", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}


fn print_usage() {
    eprintln!("{} - change occurences of one string into another",
        "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}
