use std::env;
use minigrep::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
    println!("Searching for {} in file {}", config.query, config.filename);
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
