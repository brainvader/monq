use serde::{Deserialize, Serialize};

pub type QuizID = String;

#[derive(Deserialize, Serialize, Clone)]
pub struct Quiz {
    pub id: QuizID,
    pub title: Cell,
    pub question: Vec<Cell>,
    pub answer: Vec<Cell>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Cell {
    pub r#type: String,
    pub content: String,
}
