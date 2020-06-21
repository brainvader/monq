use super::entity;

pub struct DownloadQuizRequestParams {
    pub id: entity::QuizID,
}

pub struct QuizDownloaded<T> {
    pub source: T,
}

pub trait QuizInputPort {
    type Output;
    fn download_quiz(&self, params: DownloadQuizRequestParams) -> QuizDownloaded<Self::Output>;
}
pub trait QuizOutputPort {
    fn downloaded_quiz(&self, quiz: entity::Quiz) -> QuizDownloaded<entity::Quiz>;
}

pub trait QuizRepository {
    fn find_by_id(&self, id: &entity::QuizID) -> entity::Quiz;
}

pub struct QuizInteractor<OutputPort, Repository>
where
    OutputPort: QuizOutputPort,
    Repository: QuizRepository,
{
    pub output_port: OutputPort,
    pub repository: Repository,
}

impl<OutputPort, Repository> QuizInputPort for QuizInteractor<OutputPort, Repository>
where
    OutputPort: QuizOutputPort,
    Repository: QuizRepository,
{
    type Output = entity::Quiz;
    fn download_quiz(&self, params: DownloadQuizRequestParams) -> QuizDownloaded<Self::Output> {
        let id = params.id;
        let quiz = self.repository.find_by_id(&id);
        self.output_port.downloaded_quiz(quiz)
    }
}
