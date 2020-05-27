use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_web::{web, HttpServer};
use anyhow::Context;
use listenfd::ListenFd;

use backend_lib::setup_logger;

const HOST: &'static str = "HOST";
const PORT: &'static str = "PORT";

mod endpoints;
use endpoints::{hello_monq, index, page_not_found};

fn get_env_var(key: &str) -> anyhow::Result<String> {
    let value = dotenv::var(key).with_context(|| format!("Failed to find key: {}", key))?;
    Ok(value)
}

#[actix_rt::main]
async fn start_server() -> anyhow::Result<()> {
    // local loop back address
    let host: String = get_env_var(HOST)?;
    let localhost: Ipv4Addr = host.parse::<Ipv4Addr>().unwrap();
    let ip = IpAddr::V4(localhost);
    assert_eq!(ip.is_loopback(), true);

    let port = get_env_var(PORT)?;
    let port = port.parse::<u16>().unwrap();
    let addr = SocketAddr::new(ip, port);

    // actix-web application factory
    let app_factory = move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(index)
            .service(hello_monq)
            .default_service(web::route().to(page_not_found))
    };

    let mut server = HttpServer::new(app_factory);
    let mut listenfd = ListenFd::from_env();

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(addr)?
    };

    server.run().await;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    setup_logger().with_context(|| format!("Failed to set up logger"))?;
    start_server().with_context(|| format!("Failed to start server"))?;
    Ok(())
}
