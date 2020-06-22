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

pub struct Mutations {}

#[juniper::object(Context = GraphQLContext)]
impl Mutations {
    fn crate_quiz(
        context: &GraphQLContext,
        new_quiz: interface::NewQuiz,
    ) -> FieldResult<entity::Quiz> {
        let controller = context.controller.to_owned();
        let quiz_posted = controller.post_quiz(new_quiz);
        Ok(quiz_posted.source)
    }
}
