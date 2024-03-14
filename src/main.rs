use std::{env, net::TcpStream, process::exit};

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
    let mut open_ports: Vec<u16> = vec![];

    let min_port = 0;
    let max_port = 65535;

    for port in min_port..=max_port {
        if TcpStream::connect((config.ipaddr, port)).is_ok() {
            open_ports.push(port);
        }
    }

    println!("Opened ports for {}:\n", config.ipaddr);

    open_ports.into_iter().for_each(|port| println!("{}", port));
}
