use elasticsearch::http::response::Response;
use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::indices::{IndicesCreateParts, IndicesDeleteParts};
use elasticsearch::Elasticsearch;

use anyhow::Context;
use url::Url;

use super::super::read_json;
use super::models;

const INDEX_NAME: &str = "monq";

pub fn create_elasticsearch_client(url: Url) -> Result<Elasticsearch, BuildError> {
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    Ok(Elasticsearch::new(transport))
}

pub async fn delete_monq(client: &Elasticsearch) -> anyhow::Result<serde_json::Value> {
    let parts = IndicesDeleteParts::Index(&[INDEX_NAME]);
    let response = client.indices().delete(parts).send().await?;
    let response_body = response.json::<serde_json::Value>().await?;
    Ok(response_body)
}

pub async fn create_index(
    client: &Elasticsearch,
    index_path: &str,
) -> anyhow::Result<serde_json::Value> {
    let json_str = read_json(index_path)?;
    let index_json: serde_json::Value =
        serde_json::from_str(&json_str).with_context(|| "failed to parse json")?;
    let parts = IndicesCreateParts::Index(INDEX_NAME);
    let response = client
        .indices()
        .create(parts)
        .body(index_json)
        .send()
        .await?;
    let response_body = response.json::<serde_json::Value>().await?;
    Ok(response_body)
}

pub async fn get_doc(client: &Elasticsearch, doc_id: &str) -> anyhow::Result<Response> {
    let get_parts = elasticsearch::GetParts::IndexId(INDEX_NAME, doc_id);
    let response = client.get(get_parts).send().await?;
    Ok(response)
}

pub async fn post_doc(
    client: &Elasticsearch,
    doc: &str,
) -> anyhow::Result<models::post::ResponseBody> {
    let index_parts = elasticsearch::IndexParts::Index(INDEX_NAME);
    let doc_json: serde_json::Value = serde_json::from_str(doc)?;
    let response = client.index(index_parts).body(doc_json).send().await?;
    let response_body = response.json::<models::post::ResponseBody>().await?;
    Ok(response_body)
}
