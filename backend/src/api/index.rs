use elasticsearch::indices::IndicesCreateParts;
use elasticsearch::Elasticsearch;
use serde::Serialize;

// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
// https://docs.rs/elasticsearch/7.8.0-alpha.1/elasticsearch/indices/struct.Indices.html#method.create
pub async fn create(client: &Elasticsearch, name: &str) -> anyhow::Result<String> {
    let request_body = RequestBody::default();
    let rb_str = serde_json::to_string(&request_body);
    log::info!("request_body \n {:?}", rb_str);

    let parts = IndicesCreateParts::Index(name);
    let response = client
        .indices()
        .create(parts)
        .body(request_body)
        .send()
        .await?;
    let response_body = response.text().await?;
    Ok(response_body)
}

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
        let title = Property {
            r#type: FieldType::Text,
        };
        let question = Property {
            r#type: FieldType::Text,
        };
        let answer = Property {
            r#type: FieldType::Text,
        };
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
    #[serde(flatten)]
    r#type: FieldType,
    // analyzer: String,
}

// https://www.elastic.co/guide/en/elasticsearch/reference/current/index-modules.html#index-modules-settings
// https://docs.rs/elasticsearch/7.8.0-alpha.1/elasticsearch/indices/struct.IndicesExists.html
#[derive(Serialize)]
pub struct Settings<'a> {
    analysis: SudachiAnalysis<'a>,
}

// https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-custom-analyzer.html
// https://www.elastic.co/guide/en/elasticsearch/reference/current/test-analyzer.html
#[derive(Serialize)]
pub struct SudachiAnalysis<'a> {
    tokenizer: SudachiTokenizer<'a>,
    analyzer: SudachiAnalyzer<'a>,
}

#[derive(Serialize)]
pub struct SudachiTokenizer<'a> {
    sudachi_tokenizer: SudachiTokenizerParams<'a>,
}

#[derive(Serialize)]
pub struct SudachiTokenizerParams<'a> {
    r#type: &'a str,
    mode: &'a str,
    discard_punctuation: bool,
    resources_path: &'a std::path::Path,
    settings_path: &'a std::path::Path,
}

#[derive(Serialize)]
pub struct SudachiAnalyzer<'a> {
    sudachi_analyzer: SudachiAnalyzerParams<'a>,
}

#[derive(Serialize)]
pub struct SudachiAnalyzerParams<'a> {
    tokenizer: &'a str,
    r#type: &'a str,
    char_filter: Vec<&'a str>,
    #[serde(rename = "filter")]
    token_filter: Vec<&'a str>,
}
