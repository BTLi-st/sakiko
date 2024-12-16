use sakiko::Session;
use sakiko::load_config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config file>", args[0]);
        std::process::exit(1);
    }
    let config = load_config(&args[1]).unwrap();
    let mut session = Session::new(config);
    session.run_stdio().unwrap();
}