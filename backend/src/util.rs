use anyhow::Context;
use env_logger::{DEFAULT_FILTER_ENV, DEFAULT_WRITE_STYLE_ENV};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use url::Url;

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
const ES_HOST: &str = "ES_HOST";
const ES_PORT: &str = "ES_PORT";

fn get_env_var(key: &str) -> anyhow::Result<String> {
    let value = dotenv::var(key).with_context(|| format!("Failed to find key: {}", key))?;
    Ok(value)
}

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

pub fn get_es_url() -> anyhow::Result<Url> {
    let es_host = get_env_var(ES_HOST)?;
    let es_port = get_env_var(ES_PORT)?;
    let es_addr = format!("http://{}:{}", es_host, es_port);
    Ok(Url::parse(&es_addr)?)
}
