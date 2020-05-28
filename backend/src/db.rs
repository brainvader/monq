// Reference
// https://github.com/kikei/actix-with-elasticsearch/blob/master/webapp/src/main.rs#L101
// https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/master/src/db.rs
// https://docs.rs/elasticsearch/7.7.0-alpha.1/elasticsearch/#create-a-client

use elasticsearch::http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::Elasticsearch;
use url::Url;

pub fn create_elasticsearch_client(url: Url) -> Result<Elasticsearch, BuildError> {
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    Ok(Elasticsearch::new(transport))
}

// pub fn get_pool() -> {}
