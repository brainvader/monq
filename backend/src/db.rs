// Reference
// https://github.com/kikei/actix-with-elasticsearch/blob/master/webapp/src/main.rs#L101
// https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/master/src/db.rs
// https://docs.rs/elasticsearch/7.7.0-alpha.1/elasticsearch/#create-a-client

use async_trait::async_trait;
use elasticsearch::Elasticsearch;

use super::clean::interface::ESHandle;
use shared::entity;
use shared::es::models;

#[derive(Clone)]
pub struct ESHandler {
    pub client: Elasticsearch,
}

async fn get(client: &Elasticsearch, id: &str) -> anyhow::Result<entity::Quiz> {
    let get_parts = elasticsearch::GetParts::IndexId("monq", id);
    let response = client.get(get_parts).send().await?;
    let response_body = response
        .json::<models::index::ResponseBody<entity::Quiz>>()
        .await?;
    Ok(response_body.source)
}

async fn post(client: &Elasticsearch, quiz: &entity::Quiz) -> anyhow::Result<entity::Quiz> {
    let index_parts = elasticsearch::IndexParts::IndexId("monq", &quiz.id);
    let response = client.index(index_parts).body(quiz).send().await?;
    let _ = response.json::<models::post::ResponseBody>().await?;
    let clone = quiz.to_owned();
    Ok(clone)
}

#[async_trait]
impl ESHandle for ESHandler {
    async fn get(&self, id: &str) -> entity::Quiz {
        let hoge = get(&self.client, id).await;
        // FIXME: Use anyhow or Result to handle error
        hoge.unwrap()
    }

    async fn post(&self, quiz: &entity::Quiz) -> entity::Quiz {
        post(&self.client, quiz).await.unwrap()
    }
}
