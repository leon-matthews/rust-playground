
use log::{error, warn, info, debug, trace};


fn main() {
    // Quick start: use default initialization
    colog::init();

    error!("error message");
    error!("error with fmt: {}", 42);
    warn!("warn message");
    info!("info message");
    debug!("debug message"); // not printed (LogLevel::Info is the default level)
    trace!("trace message"); // not printed (LogLevel::Info is the default level)

    // notice how multi-line comments are handled gracefully
    info!("multi line demonstration\nhere");
    info!("more\nmulti\nline\nhere\nhere");
}
