use std::{
    fmt::Display,
    net::{AddrParseError, IpAddr, Ipv4Addr},
    num::ParseIntError,
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
    BadFlag(String),
    BadThreads(u16),
    BadThreadConversion(ParseIntError),
    NoThreads,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::BadLen => write!(f, "Bad length"),
            ConfigError::BadIp(err) => write!(f, "Bad IP address: {}", err),
            ConfigError::BadFlag(val) => write!(f, "Unknown flag: {}", val),
            ConfigError::BadThreads(val) => write!(f, "Bad threads value: {}", val),
            ConfigError::NoThreads => write!(f, "No threads value provided"),
            ConfigError::BadThreadConversion(err) => {
                write!(f, "Cannot convert threads value {}", err)
            }
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

        if let Some(threads_flag) = args.next() {
            if threads_flag.eq("-j") || threads_flag.eq("--threads") {
                let Some(threads_str) = args.next() else {
                    return Err(ConfigError::NoThreads);
                };
                let threads: u16 = threads_str
                    .parse::<u16>()
                    .map_err(ConfigError::BadThreadConversion)?;

                if threads == 0 {
                    return Err(ConfigError::BadThreads(threads));
                }

                return Ok(Config {
                    help: false,
                    ipaddr,
                    threads,
                });
            }
            return Err(ConfigError::BadFlag(threads_flag));
        }

        Ok(Config {
            help: false,
            ipaddr,
            threads: 4,
        })
    }
}
