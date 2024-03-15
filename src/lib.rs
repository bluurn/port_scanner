use bpaf::Bpaf;
use std::net::{IpAddr, Ipv4Addr};

const DEFAULT_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const MIN_PORT: u16 = 1u16;
const MAX_PORT: u16 = 65535u16;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
/// Accepts IPv4 address and number of threads
pub struct Config {
    #[bpaf(short, long, fallback(IpAddr::V4(DEFAULT_IP)))]
    /// IPv4 address, defaults to 127.0.0.1
    pub ipaddr: IpAddr,

    #[bpaf(short('M'), long, fallback(MAX_PORT), guard(gtzero, "must be > 0"))]
    /// Maximal port, defaults to 65535
    pub max_port: u16,

    #[bpaf(short, long, fallback(MIN_PORT), guard(gtzero, "must be > 0"))]
    /// Minimal port, defaults to 1
    pub min_port: u16,
}

fn gtzero(input: &u16) -> bool {
    *input > 0
}
