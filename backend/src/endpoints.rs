use actix_web::{get, http, post, web, HttpRequest, HttpResponse};
use elasticsearch::http::response::Response;
use elasticsearch::Error as ES_Error;
use futures::future::TryFutureExt;

use super::clean::entity::{Cell, Quiz, QuizTitle};
use super::context::QuizController;

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

#[get("/cat")]
pub async fn cat(controller: web::Data<QuizController>) -> impl actix_web::Responder {
    let cat_api = controller.input_port.repository.handler.client.cat();
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

#[get("/quizzes/{id}")]
pub async fn get_quiz(req: HttpRequest, id: web::Path<String>) -> impl actix_web::Responder {
    let question = vec![Cell {
        r#type: "text".to_owned(),
        content: "MonQとは?".to_owned(),
    }];
    let answer = vec![Cell {
        r#type: "text".to_owned(),
        content: "MonQはクイズベースの学習システムです.".to_owned(),
    }];
    let quiz = Quiz {
        id: id.to_owned(),
        title: QuizTitle {
            r#type: "text".to_owned(),
            content: "MonQ Overview".to_owned(),
        },
        question,
        answer,
    };
    log::info!("Get Quizzes");
    log::info!("{:?}", id);
    HttpResponse::Ok().json(quiz)
}

#[post("/quizzes")]
pub async fn post_quiz(quiz: web::Json<Quiz>) -> impl actix_web::Responder {
    log::info!("Posted");
    log::info!("{:?}", quiz.id);
    let mut builder = HttpResponse::Ok();
    let mime_type = http::header::ContentType::json();
    builder
        .content_type(mime_type.to_string())
        .json(quiz.into_inner())
}

pub fn monq_endpoints(config: &mut web::ServiceConfig) {
    config.service(cat).service(get_quiz).service(post_quiz);
}
