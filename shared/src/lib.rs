use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::Elasticsearch;

use url::Url;

pub fn create_elasticsearch_client(url: Url) -> Result<Elasticsearch, BuildError> {
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    Ok(Elasticsearch::new(transport))
}
