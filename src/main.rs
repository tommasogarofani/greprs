use clap::Parser;
use greprs::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = greprs::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
