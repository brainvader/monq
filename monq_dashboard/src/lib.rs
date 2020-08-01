use seed::{prelude::*, *};

mod generated;
use generated::tailwind_classes;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

type Model = i32;

enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

fn view(_: &Model) -> Node<Msg> {
    div![
        div!["こんな感じで数式表示"],
        div![C!["math"], r#"$$ \frac{a}{b} $$"#],
        div!["コードもハイライト可能です."],
        pre![code![
            C!["rust"],
            r#"#[derive(Debug)]
    pub enum State {
        Start,
        Transient,
        Closed,
    }
    
    impl From<&'a str> for State {
        fn from(s: &'a str) -> Self {
            match s {
                "start" => State::Start,
                "closed" => State::Closed,
                _ => unreachable!(),
            }
        }
    }"#
        ]]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
