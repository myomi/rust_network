use env_logger;
use log::{error, info};
use std::env;

fn main() {
    // setup logger
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];
    info!("protocol: {}", protocol);
    info!("role: {}", role);
    info!("address: {}", address);

    // startup
    match (protocol, role) {
        ("tcp", "server") => {
            unimplemented!();
        },
        ("tcp", "client") => {
            unimplemented!();
        },
        ("tcp", _) => {
            missing_role();
        },
        ("udp", "server") => {
            unimplemented!();
        },
        ("udp", "client") => {
            unimplemented!();
        },
        ("udp", _) => {
            missing_role();
        },
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}