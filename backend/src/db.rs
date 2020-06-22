// Reference
// https://github.com/kikei/actix-with-elasticsearch/blob/master/webapp/src/main.rs#L101
// https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/master/src/db.rs
// https://docs.rs/elasticsearch/7.7.0-alpha.1/elasticsearch/#create-a-client

use async_trait::async_trait;
use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::Elasticsearch;
use url::Url;

use super::clean::entity;
use super::clean::interface::{ESHandle, IndexResponseBody, ResponseBody};

pub fn create_elasticsearch_client(url: Url) -> Result<Elasticsearch, BuildError> {
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    Ok(Elasticsearch::new(transport))
}

#[derive(Clone)]
pub struct ESHandler {
    pub client: Elasticsearch,
}

async fn get(client: &Elasticsearch, id: &str) -> anyhow::Result<entity::Quiz> {
    let get_parts = elasticsearch::GetParts::IndexId("monq", id);
    let response = client.get(get_parts).send().await?;
    let response_body = response.json::<ResponseBody<entity::Quiz>>().await?;
    Ok(response_body.source)
}

async fn post(client: &Elasticsearch, quiz: &entity::NewQuiz) -> anyhow::Result<entity::NewQuiz> {
    let index_parts = elasticsearch::IndexParts::IndexId("monq", &quiz.id);
    let response = client.index(index_parts).body(quiz).send().await?;
    let _ = response.json::<IndexResponseBody>().await?;
    let clone = quiz.clone();
    Ok(clone)
}

#[async_trait]
impl ESHandle for ESHandler {
    async fn get(&self, id: &str) -> entity::Quiz {
        let hoge = get(&self.client, id).await;
        // FIXME: Use anyhow or Result to handle error
        hoge.unwrap()
    }

    async fn post(&self, quiz: &entity::NewQuiz) -> entity::NewQuiz {
        post(&self.client, quiz).await.unwrap()
    }
}
