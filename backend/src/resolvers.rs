use super::clean::{entity, usecases};
use super::context::GraphQLContext;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::object(Context = GraphQLContext)]
impl QueryRoot {
    fn quiz(context: &GraphQLContext, id: String) -> FieldResult<entity::Quiz> {
        let controller = context.controller.clone();
        let quiz_id = entity::QuizID::from(id);
        let params = usecases::DownloadQuizRequestParams { id: quiz_id };
        let downloaded = controller.download_quiz(params);
        Ok(downloaded.source)
    }
}

pub struct MutationRoot;

#[juniper::object(Context = GraphQLContext)]
impl MutationRoot {
    fn crate_quiz(
        context: &GraphQLContext,
        new_quiz: entity::NewQuiz,
    ) -> FieldResult<entity::NewQuiz> {
        let controller = context.controller.to_owned();
        let crated = controller.post_quiz(new_quiz);
        Ok(crated.source)
    }
}
