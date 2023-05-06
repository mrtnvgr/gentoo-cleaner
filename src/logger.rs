pub fn init() {
    if std::env::var("RUST_LOG").is_err() {
        let log_level = default_log_level();
        std::env::set_var("RUST_LOG", log_level);
    }
    pretty_env_logger::init();
}

fn default_log_level() -> String {
    if cfg!(debug_assertions) {
        "debug".to_owned()
    } else {
        "info".to_owned()
    }
}
