use env_logger::{DEFAULT_FILTER_ENV, DEFAULT_WRITE_STYLE_ENV};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use shared::get_env_var;

pub fn setup_logger() -> dotenv::Result<()> {
    let log_level: String = dotenv::var(DEFAULT_FILTER_ENV)?;
    let log_style: String = dotenv::var(DEFAULT_WRITE_STYLE_ENV)?;
    std::env::set_var(DEFAULT_FILTER_ENV, log_level);
    std::env::set_var(DEFAULT_WRITE_STYLE_ENV, log_style);
    env_logger::builder().init();
    Ok(())
}

const HOST: &str = "HOST";
const PORT: &str = "PORT";

pub fn get_server_address() -> anyhow::Result<SocketAddr> {
    // local loop back address
    let host: String = get_env_var(HOST)?;
    let localhost: Ipv4Addr = host.parse::<Ipv4Addr>().unwrap();
    let ip = IpAddr::V4(localhost);
    assert_eq!(ip.is_loopback(), true);

    let port = get_env_var(PORT)?;
    let port = port.parse::<u16>().unwrap();
    let addr = SocketAddr::new(ip, port);
    Ok(addr)
}
