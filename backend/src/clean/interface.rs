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
