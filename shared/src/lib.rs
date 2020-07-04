use elasticsearch::http::response::Response;
use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::Elasticsearch;

use serde::Serialize;
use url::Url;

const INDEX_NAME: &str = "monq";

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
