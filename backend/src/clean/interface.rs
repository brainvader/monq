use super::usecases;

pub struct Controller<InputPort>
where
    InputPort: usecases::QuizInputPort,
{
    pub inputport: InputPort,
}
