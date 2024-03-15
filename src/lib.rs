use bpaf::Bpaf;
use std::net::{IpAddr, Ipv4Addr};

static DEFAULT_IP: std::net::Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
/// Accepts IPv4 address and number of threads
pub struct Config {
    /// IPv4 address, defaults to 127.0.0.1
    #[bpaf(short, long, fallback(std::net::IpAddr::V4(DEFAULT_IP)))]
    pub ipaddr: IpAddr,
    /// Number of threads, defaults to 8
    #[bpaf(short, long, fallback(8), guard(positive, "must be positive"))]
    pub threads: u16,
}

fn positive(input: &u16) -> bool {
    *input > 0
}
