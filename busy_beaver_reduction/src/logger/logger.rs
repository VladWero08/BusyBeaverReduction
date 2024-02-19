use std::env;

/// Function that will set the `RUST_LOG` environment variable
/// to use all levels of logging for the project's main executable.
pub fn load_logger() {
    let logging = "RUST_LOG";
    let logging_level = "busy_beaver_reduction=trace";

    env::set_var(logging, logging_level);
    env_logger::init();
}
