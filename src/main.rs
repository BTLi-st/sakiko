use sakiko::load_config;
/// 同步版本的运行程序
/// 通过标准输入输出进行交互
/// 用法: cargo run --release -- <config file>
use sakiko::Session;
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
