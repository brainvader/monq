use actix_web::{web, HttpServer};
use anyhow::Context;
use listenfd::ListenFd;
use url::Url;

use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::Elasticsearch;

const ES_HOST: &'static str = "ES_HOST";
const ES_PORT: &'static str = "ES_PORT";

mod endpoints;
use endpoints::{cat, hello_monq, index, page_not_found};

use backend_lib::{get_env_var, get_server_address, setup_logger};

fn create_elasticsearch_client(url: Url) -> Result<Elasticsearch, BuildError> {
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    Ok(Elasticsearch::new(transport))
}

#[actix_rt::main]
async fn start_server(client: Elasticsearch) -> anyhow::Result<()> {
    let addr = get_server_address()?;
    // actix-web application factory
    let app_factory = move || {
        actix_web::App::new()
            .data(client.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(index)
            .service(hello_monq)
            .service(cat)
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

    let es_host = get_env_var(ES_HOST)?;
    let es_port = get_env_var(ES_PORT)?;
    let es_addr = format!("http://{}:{}", es_host, es_port);
    let url = Url::parse(&es_addr)?;
    let client = create_elasticsearch_client(url)
        .with_context(|| format!("Failed to create elasticsearch client"))?;

    start_server(client).with_context(|| format!("Failed to start server"))?;
    Ok(())
}
