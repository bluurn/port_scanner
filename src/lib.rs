use std::{
    fmt::Display,
    net::{AddrParseError, IpAddr, Ipv4Addr},
    str::FromStr,
};

#[derive(Debug)]
pub struct Config {
    pub help: bool,
    pub ipaddr: IpAddr,
    pub threads: u16,
}

#[derive(Debug)]
pub enum ConfigError {
    BadLen,
    BadIp(AddrParseError),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::BadLen => write!(f, "Bad length"),
            ConfigError::BadIp(err) => write!(f, "Bad IP address: {}", err),
        }
    }
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        args.next();

        let Some(fst_arg) = args.next() else {
            return Err(ConfigError::BadLen);
        };

        if fst_arg.eq("-h") || fst_arg.eq("--help") {
            return Ok(Config {
                help: true,
                ipaddr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                threads: 0,
            });
        }

        let ipaddr = IpAddr::from_str(&fst_arg).map_err(ConfigError::BadIp)?;

        Ok(Config {
            help: false,
            ipaddr,
            threads: 4,
        })
    }
}
