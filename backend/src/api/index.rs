use serde::Serialize;

pub struct Index<'a> {
    name: &'a str,
    config: RequestBody,
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
pub struct RequestBody {
    mappings: Mappings,
}

impl Default for RequestBody {
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
pub struct Mappings {
    properties: Properties,
}

#[derive(Serialize)]
pub struct Properties {
    title: Property,
    question: Property,
    answer: Property,
}

// Mapping parameters
// https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-params.html
#[derive(Serialize)]
pub struct Property {
    r#type: FieldType,
    // analyzer: String,
}
