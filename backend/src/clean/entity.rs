use serde::{Deserialize, Serialize};

pub type QuizID = String;
pub type QuizTitle = Cell;
pub type QuizQuestion = Vec<Cell>;
pub type QuizAnswer = Vec<Cell>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Quiz {
    pub id: QuizID,
    pub title: QuizTitle,
    pub question: QuizQuestion,
    pub answer: QuizAnswer,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Cell {
    pub r#type: String,
    pub content: String,
}
