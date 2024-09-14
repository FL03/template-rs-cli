/*
    Appellation: proton-cli <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Proton CLI
// extern crate proton_sdk as proton;

#[macro_use]
pub(crate) mod macros;

pub mod cmd;
pub mod cnf;

fn main() {
    init_tracing("debug");
    let _cli = cmd::new();
}

fn init_tracing(level: impl ToString) {
    use tracing::Level;
    let level = match level.to_string().to_lowercase().as_str() {
        "debug" => Level::DEBUG,
        "error" => Level::ERROR,
        "trace" => Level::TRACE,
        "warn" => Level::WARN,
        _ => Level::INFO,
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();
    tracing::info!("Success: initialized the tracing modules...");
}
