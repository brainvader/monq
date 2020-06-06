use elasticsearch::Elasticsearch;

pub struct GraphQLContext {
    pub client: Elasticsearch,
}

impl juniper::Context for GraphQLContext {}
