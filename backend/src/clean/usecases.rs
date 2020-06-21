use super::entity;

pub struct DownloadQuizRequestParams {
    pub id: entity::QuizID,
}

pub struct QuizDownloaded<T> {
    pub source: T,
}

pub struct QuizPosted<T> {
    pub source: T,
}

pub trait QuizInputPort {
    type Output1;
    type Output2;
    fn download_quiz(&self, params: DownloadQuizRequestParams) -> QuizDownloaded<Self::Output1>;
    fn post_quiz(&self, quiz: entity::NewQuiz) -> QuizPosted<Self::Output2>;
}
pub trait QuizOutputPort {
    fn downloaded_quiz(&self, quiz: entity::Quiz) -> QuizDownloaded<entity::Quiz>;
    fn post_quiz(&self, quiz: entity::NewQuiz) -> QuizPosted<entity::NewQuiz>;
}

pub trait QuizRepository {
    fn find_by_id(&self, id: &entity::QuizID) -> entity::Quiz;
    fn create(&self, quiz: &entity::NewQuiz) -> entity::NewQuiz;
}

#[derive(Clone)]
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
    type Output1 = entity::Quiz;
    type Output2 = entity::NewQuiz;
    fn download_quiz(&self, params: DownloadQuizRequestParams) -> QuizDownloaded<Self::Output1> {
        let id = params.id;
        let quiz = self.repository.find_by_id(&id);
        self.output_port.downloaded_quiz(quiz)
    }

    fn post_quiz(&self, quiz: entity::NewQuiz) -> QuizPosted<Self::Output2> {
        let quiz = self.repository.create(&quiz);
        self.output_port.post_quiz(quiz)
    }
}
