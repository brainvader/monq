// FIXME: Leak entity from domain layer
use super::entity;
use super::usecases;

use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub struct ResponseBody<T> {
    #[serde(alias = "_index")]
    index: String,
    #[serde(alias = "_type")]
    r#type: String,
    #[serde(alias = "_id")]
    id: String,
    #[serde(alias = "_version")]
    version: i64,
    #[serde(alias = "_seq_no")]
    seq_no: i64,
    #[serde(alias = "_primary_term")]
    primary_term: i64,
    found: bool,
    #[serde(alias = "_source")]
    pub source: T,
}

pub trait ESHandle {
    fn get(&self, id: &entity::QuizID) -> ResponseBody<entity::Quiz>;
}

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
    fn find_by_id(&self, id: &entity::QuizID) -> entity::Quiz {
        let response = self.handler.get(id);
        response.source
    }
}
