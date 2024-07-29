
fn main() {
    let config = headr::Config::parse_args();
    if let Err(e) = headr::run(config) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
