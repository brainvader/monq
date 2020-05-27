use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_web::{web, HttpServer};
use listenfd::ListenFd;

use backend_lib::setup_logger;

const HOST: &'static str = "HOST";
const PORT: &'static str = "PORT";

mod endpoints;
use endpoints::{hello_monq, index, page_not_found};

#[actix_rt::main]
async fn start_server() -> std::io::Result<()> {
    // local loop back address
    let host: String = dotenv::var(HOST).expect("ES_HOST could not resolved");
    let localhost: Ipv4Addr = host.parse::<Ipv4Addr>().unwrap();
    let ip = IpAddr::V4(localhost);
    assert_eq!(ip.is_loopback(), true);

    let port = dotenv::var(PORT).expect("ES_PORT could not resolved");
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

    server.run().await
}

fn main() -> std::io::Result<()> {
    setup_logger().expect("Failed to set up logger");
    Ok(start_server()?)
}
