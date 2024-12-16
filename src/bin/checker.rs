use std::env;
use sakiko::SakikoConfig;
use sakiko::check::check_config;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config file>", args[0]);
        std::process::exit(1);
    }
    let config = SakikoConfig::deserialize_from_file(&args[1]).unwrap();
    match check_config(&config) {
        Ok(_) => println!("Config is valid"),
        Err(e) => eprintln!("Config is invalid: {}", e),
    }
}