// Reference
// https://github.com/kikei/actix-with-elasticsearch/blob/master/webapp/src/main.rs#L101
// https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/master/src/db.rs
// https://docs.rs/elasticsearch/7.7.0-alpha.1/elasticsearch/#create-a-client

use async_trait::async_trait;
use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::Elasticsearch;
use url::Url;

use super::clean::entity;
use super::clean::interface::{ESHandle, ResponseBody};

pub fn create_elasticsearch_client(url: Url) -> Result<Elasticsearch, BuildError> {
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    Ok(Elasticsearch::new(transport))
}

pub struct ESHandler {
    pub client: Elasticsearch,
}

async fn get(client: &Elasticsearch, id: &entity::QuizID) -> anyhow::Result<entity::Quiz> {
    let get_parts = elasticsearch::GetParts::IndexId("monq", id);
    let response = client.get(get_parts).send().await?;
    let response_body = response.json::<ResponseBody<entity::Quiz>>().await?;
    Ok(response_body.source)
}

#[async_trait]
impl ESHandle for ESHandler {
    async fn get(&self, id: &entity::QuizID) -> entity::Quiz {
        let hoge = get(&self.client, id).await;
        // FIXME: Use anyhow or Result to handle error
        hoge.unwrap()
    }
}
