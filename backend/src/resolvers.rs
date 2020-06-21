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
