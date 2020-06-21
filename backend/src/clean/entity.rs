use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

pub type QuizID = juniper::ID;
pub type QuizTitle = Cell;
pub type QuizQuestion = Vec<Cell>;
pub type QuizAnswer = Vec<Cell>;

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

pub type NewID = juniper::ID;
pub type NewTitle = NewCell;
pub type NewQuestion = Vec<NewCell>;
pub type NewAnswer = Vec<NewCell>;

#[derive(GraphQLInputObject, Deserialize, Serialize, Clone)]
pub struct NewQuiz {
    pub id: NewID,
    pub title: NewTitle,
    pub question: NewQuestion,
    pub answer: NewAnswer,
}

#[derive(GraphQLInputObject, Deserialize, Serialize, Clone)]
pub struct NewCell {
    pub r#type: String,
    pub content: String,
}
