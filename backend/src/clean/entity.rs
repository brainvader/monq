pub type QuizID = juniper::ID;
type QuizTitle = String;
type QuizQuestion = Vec<Cell>;
type QuizAnswer = Vec<Cell>;

pub struct Quiz {
    pub id: QuizID,
    pub title: QuizTitle,
    pub question: QuizQuestion,
    pub answer: QuizAnswer,
}

pub struct Cell {
    pub r#type: String,
    pub content: String,
}
