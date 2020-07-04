use elasticsearch::http::response::Response;
use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::Elasticsearch;

use anyhow::Context;
use serde::Serialize;
use url::Url;

const INDEX_NAME: &str = "monq";
const ES_HOST: &str = "ES_HOST";
const ES_PORT: &str = "ES_PORT";

pub fn get_env_var(key: &str) -> anyhow::Result<String> {
    let value = dotenv::var(key).with_context(|| format!("Failed to find key: {}", key))?;
    Ok(value)
}

pub fn get_es_url() -> anyhow::Result<Url> {
    let es_host = get_env_var(ES_HOST)?;
    let es_port = get_env_var(ES_PORT)?;
    let es_addr = format!("http://{}:{}", es_host, es_port);
    Ok(Url::parse(&es_addr)?)
}

pub fn create_elasticsearch_client(url: Url) -> Result<Elasticsearch, BuildError> {
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    Ok(Elasticsearch::new(transport))
}

pub async fn get_doc(client: &Elasticsearch, doc_id: &str) -> anyhow::Result<Response> {
    let get_parts = elasticsearch::GetParts::IndexId(INDEX_NAME, doc_id);
    let response = client.get(get_parts).send().await?;
    Ok(response)
}

pub async fn post_doc<T>(client: &Elasticsearch, doc: &T, doc_id: &str) -> anyhow::Result<Response>
where
    T: Serialize,
{
    let index_parts = elasticsearch::IndexParts::IndexId(INDEX_NAME, doc_id);
    let response = client.index(index_parts).body(doc).send().await?;
    Ok(response)
}
