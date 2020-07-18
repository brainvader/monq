use actix_files::Files;
use actix_web::{web, HttpServer};
use anyhow::Context;
use elasticsearch::Elasticsearch;
use listenfd::ListenFd;

use backend_lib::api;
use backend_lib::clean::{interface, usecases};
use backend_lib::db::ESHandler;
use backend_lib::endpoints::{dashboard, hello_monq, index, monq_endpoints, page_not_found};
use backend_lib::util::get_server_address;

use shared::es::api::create_elasticsearch_client;
use shared::{get_es_url, setup_logger};

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
            .service(dashboard)
            .service(Files::new("/pkg", "./monq_dashboard/pkg"))
            .service(Files::new("/public", "./monq_dashboard/public"))
            .service(web::scope("/api").configure(monq_endpoints))
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
    // let mut executor =
    //     tokio::runtime::Runtime::new().with_context(|| "Failed to crate tokio runtime")?;
    // let request_body = api::index::RequestBody::default();
    // let monq = api::index::Index {
    //     name: "monq",
    //     config: request_body,
    // };
    // let response = executor
    //     .block_on(api::index::create(&client, monq))
    //     .with_context(|| "Failed to create index")?;
    // log::info!("{}", response);

    start_server(client).with_context(|| "Failed to start server")?;
    Ok(())
}
