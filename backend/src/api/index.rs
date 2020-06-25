use serde::Serialize;

pub struct Index<'a> {
    name: &'a str,
    config: RequestBody<'a>,
}

// https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum FieldType {
    Text,
    Keyword,
    Date,
    Long,
    Double,
    Boolean,
}

#[derive(Serialize)]
pub struct RequestBody<'a> {
    mappings: Mappings<'a>,
}

impl<'a> Default for RequestBody<'a> {
    fn default() -> Self {
        let title = Property { r#type: "text" };
        let question = Property { r#type: "text" };
        let answer = Property { r#type: "text" };
        let properties = Properties {
            title,
            question,
            answer,
        };
        let mappings = Mappings { properties };
        Self { mappings }
    }
}

#[derive(Serialize)]
// #[serde(rename = "mappings")]
pub struct Mappings<'a> {
    properties: Properties<'a>,
}

#[derive(Serialize)]
pub struct Properties<'a> {
    title: Property<'a>,
    question: Property<'a>,
    answer: Property<'a>,
}

// Mapping parameters
// https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-params.html
#[derive(Serialize)]
pub struct Property<'a> {
    r#type: &'a str,
    // analyzer: String,
}
