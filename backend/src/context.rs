use super::clean::interface::{Controller, QuizDocumentRepository, QuizPresenter};
use super::clean::usecases::QuizInteractor;
use super::db::ESHandler;

pub type QuizController =
    Controller<QuizInteractor<QuizPresenter, QuizDocumentRepository<ESHandler>>>;

pub struct GraphQLContext {
    pub controller: QuizController,
}

impl juniper::Context for GraphQLContext {}
