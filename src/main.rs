use std::{
    env,
    net::{IpAddr, TcpStream},
    process::exit,
    sync::mpsc::{channel, Sender},
    thread,
};

fn print_usage() {
    let prg_name = env::args().next().unwrap_or("prog_name".to_string());
    println!("Usage: {} -h or --help to print help", prg_name);
    println!("{} 127.0.0.1 to scan ports", prg_name);
    println!("{} 127.0.0.1 -j 8 to scan ports with 8 threads", prg_name);
}

fn run(config: port_scanner::Config) {
    let mut open_ports: Vec<u16> = vec![];
    let (tx, rx) = channel::<u16>();

    for thread_num in 0..config.threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan_port(tx, thread_num, config.ipaddr, config.threads);
        });
    }

    drop(tx);

    rx.into_iter().for_each(|received| {
        open_ports.push(received);
    });

    println!("Opened ports:");

    open_ports.into_iter().for_each(|port| println!("{}", port));
}

fn scan_port(tx: Sender<u16>, thread_num: u16, ipaddr: IpAddr, thread_count: u16) {
    let max_port = 65535;
    let mut curr_port = thread_num;

    while max_port - curr_port >= thread_count {
        if TcpStream::connect((ipaddr, curr_port)).is_ok() {
            tx.send(curr_port).unwrap();
        }
        curr_port += thread_count;
    }
}

fn main() {
    let config = port_scanner::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}", err);
        print_usage();
        exit(1);
    });

    if config.help {
        print_usage();
        exit(0);
    }

    run(config);
}
