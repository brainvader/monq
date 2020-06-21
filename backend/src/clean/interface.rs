// FIXME: Leak entity from domain layer
use super::entity;
use super::usecases;

use async_trait::async_trait;
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

    pub fn post_quiz(&self, quiz: entity::Quiz) -> usecases::QuizPosted<InputPort::Output> {
        self.inputport.post_quiz(quiz)
    }
}

pub struct QuizPresenter;

impl usecases::QuizOutputPort for QuizPresenter {
    fn downloaded_quiz(&self, quiz: entity::Quiz) -> usecases::QuizDownloaded<entity::Quiz> {
        usecases::QuizDownloaded { source: quiz }
    }

    fn post_quiz(&self, quiz: entity::Quiz) -> usecases::QuizPosted<entity::Quiz> {
        usecases::QuizPosted { source: quiz }
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

#[derive(Deserialize)]
struct Shard {
    failed: i64,
    successful: i64,
    total: i64,
}

#[derive(Deserialize)]
pub struct IndexResponseBody {
    #[serde(alias = "_index")]
    index: String,
    #[serde(alias = "_type")]
    r#type: String,
    #[serde(alias = "_id")]
    id: String,
    #[serde(alias = "_version")]
    version: i64,
    result: String,
    #[serde(alias = "_shards")]
    shards: Shard,
    #[serde(alias = "_seq_no")]
    seq_no: i64,
    #[serde(alias = "_primary_term")]
    primary_term: i64,
}

#[async_trait]
pub trait ESHandle {
    async fn get(&self, id: &entity::QuizID) -> entity::Quiz;
    async fn post(&self, quiz: &entity::Quiz) -> entity::Quiz;
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
        // TODO: Need async functions for other parts
        futures::executor::block_on(self.handler.get(id))
    }

    fn create(&self, quiz: &entity::Quiz) -> entity::Quiz {
        futures::executor::block_on(self.handler.post(quiz))
    }
}
