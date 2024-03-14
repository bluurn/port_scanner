use std::{env, process::exit};

fn print_usage() {
    let prg_name = env::args().next().unwrap_or("prog_name".to_string());
    println!("Usage: {} -h or --help to print help", prg_name);
    println!("{} 127.0.0.1 to scan ports", prg_name);
}

fn main() {
    let config = port_scanner::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}", err);
        print_usage();
        exit(1);
    });

    if config.help {
        print_usage();
    }

    run(config);
}

fn run(config: port_scanner::Config) {
    todo!("Not implemented")
}
