use anyhow::Context;
use log::info;
use url::Url;

pub mod entity;
pub mod es;

use env_logger::{DEFAULT_FILTER_ENV, DEFAULT_WRITE_STYLE_ENV};

const ES_HOST: &str = "ES_HOST";
const ES_PORT: &str = "ES_PORT";

pub fn setup_logger() -> dotenv::Result<()> {
    let log_level: String = dotenv::var(DEFAULT_FILTER_ENV)?;
    let log_style: String = dotenv::var(DEFAULT_WRITE_STYLE_ENV)?;
    std::env::set_var(DEFAULT_FILTER_ENV, log_level);
    std::env::set_var(DEFAULT_WRITE_STYLE_ENV, log_style);
    env_logger::builder().init();
    Ok(())
}

pub fn log_info(log_message: &str) {
    info!("{}", log_message);
}

pub fn get_env_var(key: &str) -> anyhow::Result<String> {
    let value = dotenv::var(key).with_context(|| format!("Failed to find key: {}", key))?;
    Ok(value)
}

pub fn run_docker_compose() -> anyhow::Result<std::process::ExitStatus> {
    let mut docker_compose = std::process::Command::new("docker-compose");
    let status = docker_compose
        .args(&["-f", "docker/docker-compose.yaml", "up", "-d"])
        .status()
        .with_context(|| "failed to get docker-compose up and running")?;
    Ok(status)
}

pub fn get_es_url() -> anyhow::Result<Url> {
    let es_host = get_env_var(ES_HOST)?;
    let es_port = get_env_var(ES_PORT)?;
    let es_addr = format!("http://{}:{}", es_host, es_port);
    Ok(Url::parse(&es_addr)?)
}

pub fn read_json<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<String> {
    let data = std::fs::read_to_string(path).with_context(|| "failed to read json")?;
    Ok(data)
}
