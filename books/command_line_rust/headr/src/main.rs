
use headr::config::Config;


fn main() {
    let config = Config::from_args();
    if let Err(e) = headr::run(config) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
