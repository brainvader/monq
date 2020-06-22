use super::clean::{entity, interface};
use super::context::GraphQLContext;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::object(Context = GraphQLContext)]
impl QueryRoot {
    fn quiz(context: &GraphQLContext, id: String) -> FieldResult<entity::Quiz> {
        let controller = context.controller.clone();
        let downloaded = controller.download_quiz(id);
        Ok(downloaded.source)
    }
}
