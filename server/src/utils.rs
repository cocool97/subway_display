use env_logger::Env;

/// Sets up the logger based on the verbosity level.
/// `RUST_LOG` variable takes precedences over the verbosity flag.
pub fn setup_logger(verbosity: u8) {
    let log_level = match verbosity {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    env_logger::init_from_env(Env::new().default_filter_or(log_level));
}
