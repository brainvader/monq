use super::entity;

pub struct DownloadQuizRequestParams {
    pub id: String,
}

pub struct PostParams {
    pub quiz: entity::Quiz,
}

pub struct QuizDownloaded {
    pub source: entity::Quiz,
}

pub struct QuizPosted {
    pub source: entity::Quiz,
}

pub trait QuizInputPort {
    fn download_quiz(&self, params: DownloadQuizRequestParams) -> QuizDownloaded;
    fn post_quiz(&self, params: PostParams) -> QuizPosted;
}
pub trait QuizOutputPort {
    fn downloaded_quiz(&self, quiz: entity::Quiz) -> QuizDownloaded;
    fn post_quiz(&self, quiz: entity::Quiz) -> QuizPosted;
}

pub trait QuizRepository {
    fn find_by_id(&self, id: &str) -> entity::Quiz;
    fn create(&self, quiz: &entity::Quiz) -> entity::Quiz;
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
    fn download_quiz(&self, params: DownloadQuizRequestParams) -> QuizDownloaded {
        let id = params.id;
        let quiz = self.repository.find_by_id(&id);
        self.output_port.downloaded_quiz(quiz)
    }

    fn post_quiz(&self, params: PostParams) -> QuizPosted {
        let quiz = params.quiz;
        let quiz = self.repository.create(&quiz);
        self.output_port.post_quiz(quiz)
    }
}
