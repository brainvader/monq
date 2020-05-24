use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_web::{web, HttpResponse, HttpServer};
use listenfd::ListenFd;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // local loop back address
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let ip = IpAddr::V4(localhost);
    assert_eq!(ip.is_loopback(), true);

    let port = 8080;
    let addr = SocketAddr::new(ip, port);

    // actix-web application factory
    let app_factory = move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .default_service(web::to(|| HttpResponse::NotFound()))
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
