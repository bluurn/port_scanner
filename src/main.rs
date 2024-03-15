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
    let (tx, mut rx) = mpsc::channel::<u16>(50);

    for port in args.min_port..=args.max_port {
        let tx = tx.clone();
        tokio::spawn(async move {
            if (TcpStream::connect((args.ipaddr, port)).await).is_ok() {
                tx.send(port).await.unwrap();
            };
        });
    }

    drop(tx);

    while let Some(port) = rx.recv().await {
        open_ports.push(port);
    }

    println!("Opened ports:");
    for port in open_ports.into_iter() {
        println!("{}", port)
    }
}
