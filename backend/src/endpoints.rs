use actix_web::{get, http, HttpResponse};

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

pub async fn page_not_found() -> impl actix_web::Responder {
    let mut builder = HttpResponse::NotFound();
    let mime_type = http::header::ContentType::plaintext();
    builder
        .content_type(mime_type.to_string())
        .body("404 Page Not Found")
}
