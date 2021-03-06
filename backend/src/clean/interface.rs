// FIXME: Leak entity from domain layer
use super::usecases;
use shared::entity;

use async_trait::async_trait;

#[derive(Clone)]
pub struct Controller<InputPort>
where
    InputPort: usecases::QuizInputPort,
{
    pub input_port: InputPort,
}

impl<InputPort> Controller<InputPort>
where
    InputPort: usecases::QuizInputPort,
{
    pub fn download_quiz(&self, id: String) -> usecases::QuizDownloaded {
        let params = usecases::DownloadQuizRequestParams { id };
        self.input_port.download_quiz(params)
    }

    pub fn post_quiz(&self, quiz: entity::Quiz) -> usecases::QuizPosted {
        let params = usecases::PostParams { quiz };
        self.input_port.post_quiz(params)
    }
}

#[derive(Clone)]
pub struct QuizPresenter;

impl usecases::QuizOutputPort for QuizPresenter {
    fn downloaded_quiz(&self, quiz: entity::Quiz) -> usecases::QuizDownloaded {
        usecases::QuizDownloaded { source: quiz }
    }

    fn post_quiz(&self, quiz: entity::Quiz) -> usecases::QuizPosted {
        usecases::QuizPosted { source: quiz }
    }
}

#[async_trait]
pub trait ESHandle {
    async fn get(&self, id: &str) -> entity::Quiz;
    async fn post(&self, quiz: &entity::Quiz) -> entity::Quiz;
}

#[derive(Clone)]
pub struct QuizDocumentRepository<Handler>
where
    Handler: ESHandle,
{
    pub handler: Handler,
}

impl<Handler> usecases::QuizRepository for QuizDocumentRepository<Handler>
where
    Handler: ESHandle,
{
    fn find_by_id(&self, id: &str) -> entity::Quiz {
        // TODO: Need async functions for other parts
        futures::executor::block_on(self.handler.get(id))
    }

    fn create(&self, quiz: &entity::Quiz) -> entity::Quiz {
        futures::executor::block_on(self.handler.post(quiz))
    }
}
