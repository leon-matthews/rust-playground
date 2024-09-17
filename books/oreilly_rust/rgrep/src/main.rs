fn main() {
    let result = rgrep::command_line();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
