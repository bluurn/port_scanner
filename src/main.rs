use std::{
    net::TcpStream,
    sync::mpsc::{channel, Sender},
    thread,
};

use port_scanner::config;

fn run(config: port_scanner::Config) {
    let mut open_ports: Vec<u16> = vec![];
    let (tx, rx) = channel::<u16>();

    for thread_num in 0..config.threads {
        let tx = tx.clone();
        let cfg = config.clone();
        thread::spawn(move || {
            scan_port(tx, thread_num, cfg);
        });
    }

    drop(tx);

    for received in rx.into_iter() {
        open_ports.push(received);
    }

    println!("Opened ports:");

    for port in open_ports.into_iter() {
        println!("{}", port)
    }
}

fn scan_port(tx: Sender<u16>, thread_num: u16, config: port_scanner::Config) {
    let max_port = 65535;
    let mut curr_port = thread_num;

    while max_port - curr_port >= config.threads {
        if TcpStream::connect((config.ipaddr, curr_port)).is_ok() {
            tx.send(curr_port).unwrap();
        }
        curr_port += config.threads;
    }
}

fn main() {
    let args = config().run();

    run(args);
}
