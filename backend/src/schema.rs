use juniper::ID;
use juniper::{FieldResult, GraphQLInputObject, GraphQLObject, RootNode};

use serde::Deserialize;

use super::context::GraphQLContext;

#[derive(GraphQLObject, Clone, Deserialize, Debug)]
#[graphql(description = "A human being in the Rebuild of Evangelion")]
pub struct Human {
    pub id: ID,
    pub name: String,
}

#[derive(GraphQLInputObject)]
struct NewHuman {
    id: ID,
    name: String,
}

pub struct QueryRoot;

#[juniper::object(Context = GraphQLContext)]
impl QueryRoot {
    fn human() -> FieldResult<Human> {
        Ok(Human {
            id: ID::from("1234".to_owned()),
            name: "Luke".to_owned(),
        })
    }
}
pub struct MutationRoot;

#[juniper::object(Context = GraphQLContext)]
impl MutationRoot {
    fn createHuman(new_human: NewHuman) -> FieldResult<Human> {
        let human = Human {
            id: new_human.id.to_owned(),
            name: new_human.name.to_owned(),
        };
        log::info!("{:?}", human);
        Ok(human)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
