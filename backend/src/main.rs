use actix_web::{web, HttpServer};
use anyhow::Context;
use elasticsearch::Elasticsearch;
use listenfd::ListenFd;

use backend_lib::clean::{interface, usecases};
use backend_lib::db::create_elasticsearch_client;
use backend_lib::db::ESHandler;
use backend_lib::endpoints::{cat, graphql_config, hello_monq, index, page_not_found};
use backend_lib::util::{get_es_url, get_server_address, setup_logger};

#[actix_rt::main]
async fn start_server(client: Elasticsearch) -> anyhow::Result<()> {
    let handler = ESHandler { client };
    let repository = interface::QuizDocumentRepository { handler };
    let output_port = interface::QuizPresenter;
    let input_port = usecases::QuizInteractor {
        output_port,
        repository,
    };
    let controller = interface::Controller { input_port };

    let addr = get_server_address()?;
    // actix-web application factory
    let app_factory = move || {
        actix_web::App::new()
            .data(controller.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(index)
            .service(hello_monq)
            .service(cat)
            .configure(graphql_config)
            .default_service(web::route().to(page_not_found))
    };

    let mut server = HttpServer::new(app_factory);
    let mut listenfd = ListenFd::from_env();

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(addr)?
    };

    server.run().await?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    setup_logger().with_context(|| "Failed to set up logger")?;
    let url = get_es_url()?;
    let client = create_elasticsearch_client(url)
        .with_context(|| "Failed to create elasticsearch client")?;

    start_server(client).with_context(|| "Failed to start server")?;
    Ok(())
}
