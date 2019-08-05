use env_logger;
use log::{debug, error, info, warn};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    info!("Hello, logger");
    error!("error");
    warn!("warning!");
    debug!("debug!!");
}
