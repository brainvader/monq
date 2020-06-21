use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

pub type QuizID = juniper::ID;
type QuizTitle = Cell;
type QuizQuestion = Vec<Cell>;
type QuizAnswer = Vec<Cell>;

#[derive(GraphQLObject, Deserialize, Serialize, Clone)]
#[graphql(description = "Quiz consists of question and answer")]
pub struct Quiz {
    pub id: QuizID,
    pub title: QuizTitle,
    pub question: QuizQuestion,
    pub answer: QuizAnswer,
}

#[derive(GraphQLObject, Deserialize, Serialize, Clone)]
#[graphql(description = "A cell contains various kinds of format data")]
pub struct Cell {
    pub r#type: String,
    pub content: String,
}
