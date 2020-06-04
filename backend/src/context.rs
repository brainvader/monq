use elasticsearch::Elasticsearch;

pub struct Context {
    pub client: Elasticsearch,
}

impl juniper::Context for Context {}
