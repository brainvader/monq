// FIXME: Leak entity from domain layer
use super::entity;
use super::usecases;

pub struct Controller<InputPort>
where
    InputPort: usecases::QuizInputPort,
{
    pub inputport: InputPort,
}

impl<InputPort> Controller<InputPort>
where
    InputPort: usecases::QuizInputPort,
{
    pub fn download_quiz(
        &self,
        params: usecases::DownloadQuizRequestParams,
    ) -> usecases::QuizDownloaded<InputPort::Output> {
        self.inputport.download_quiz(params)
    }
}

pub struct QuizPresenter {}

impl usecases::QuizOutputPort for QuizPresenter {
    fn downloaded_quiz(&self, quiz: entity::Quiz) -> usecases::QuizDownloaded<entity::Quiz> {
        usecases::QuizDownloaded { source: quiz }
    }
}
