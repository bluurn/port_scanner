use indicatif::ProgressBar;
use port_scanner::config;
use std::process::exit;
use tokio::{net::TcpStream, sync::mpsc};

#[tokio::main]
async fn main() {
    let args = config().run();

    if args.min_port >= args.max_port {
        eprintln!("Min port should be less than max port!");
        exit(1);
    }

    let mut open_ports: Vec<u16> = vec![];
    let (tx, mut rx) = mpsc::channel::<port_scanner::Message>(50);
    let pb = ProgressBar::new(args.max_port.into());
    for port in args.min_port..=args.max_port {
        let tx = tx.clone();
        tokio::spawn(async move {
            let is_open = (TcpStream::connect((args.ipaddr, port)).await).is_ok();
            let _ = tx.send(port_scanner::Message { port, is_open }).await;
        });
    }

    drop(tx);

    while let Some(msg) = rx.recv().await {
        if msg.is_open {
            open_ports.push(msg.port);
        }

        pb.inc(1)
    }

    open_ports.sort();
    pb.finish_and_clear();

    println!("Opened ports for {}:", args.ipaddr);
    for port in open_ports.into_iter() {
        println!("{}", port)
    }
}
