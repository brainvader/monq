use actix_web::{get, http, post, web, Error, HttpResponse};
use elasticsearch::http::response::Response;
use elasticsearch::Elasticsearch as ES_Client;
use elasticsearch::Error as ES_Error;
use futures::future::TryFutureExt;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use serde_json;

use std::sync::Arc;

use super::context::GraphQLContext;
use super::schema::Schema;
use super::util::get_server_address;

#[get("/")]
pub async fn index() -> impl actix_web::Responder {
    let mut builder = HttpResponse::Ok();
    let mime_type: http::header::ContentType = http::header::ContentType::plaintext();
    builder.content_type(mime_type.to_string()).body("Welcom!!")
}

#[get("/hello-monq")]
pub async fn hello_monq() -> impl actix_web::Responder {
    let mut builder = HttpResponse::Ok();
    let mime_type: http::header::ContentType = http::header::ContentType::plaintext();
    builder
        .content_type(mime_type.to_string())
        .body("Hello MonQ!")
}

#[get("/cat")]
pub async fn cat(client: web::Data<ES_Client>) -> impl actix_web::Responder {
    let cat_api = client.get_ref().cat();
    let result: Result<String, ES_Error> = cat_api
        .nodes()
        .h(&["ip", "port", "heapPercent", "name"])
        .format("json")
        .send()
        .and_then(|response: Response| async { response.text().await })
        .await;

    match result {
        Ok(response_body) => {
            log::info!("response body: {:#?}", response_body);
            let mut builder = HttpResponse::Ok();
            let mime_type: http::header::ContentType = http::header::ContentType::json();
            builder
                .content_type(mime_type.to_string())
                .json(response_body)
        }
        Err(e) => {
            log::info!("failed to cat, error: {:?}", &e);
            HttpResponse::NotFound().finish()
        }
    }
}

pub async fn page_not_found() -> impl actix_web::Responder {
    let mut builder = HttpResponse::NotFound();
    let mime_type = http::header::ContentType::plaintext();
    builder
        .content_type(mime_type.to_string())
        .body("404 Page Not Found")
}

#[get("/graphiql")]
pub async fn graphiql() -> impl actix_web::Responder {
    match get_server_address() {
        Ok(addr) => {
            let addr_str = addr.to_string();
            let html = graphiql_source(&format!("http://{}/graphql", addr_str));
            // TODO: Add plain and CORS header
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
        }
        Err(e) => {
            log::info!("Failed to get server address: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/graphql")]
pub async fn graphql(
    state: web::Data<ES_Client>,
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let client = state.get_ref().to_owned();
    let context = GraphQLContext { client: client };
    let res = web::block(move || {
        let res = data.execute(&schema, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}
