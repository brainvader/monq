use super::resolvers::{MutationRoot, QueryRoot};
use juniper::RootNode;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
