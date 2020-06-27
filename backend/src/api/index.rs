use elasticsearch::indices::IndicesCreateParts;
use elasticsearch::Elasticsearch;
use serde::Serialize;

// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
// https://docs.rs/elasticsearch/7.8.0-alpha.1/elasticsearch/indices/struct.Indices.html#method.create
pub async fn create<'a>(client: &Elasticsearch, index: Index<'a>) -> anyhow::Result<String> {
    let request_body = index.config;
    let rb_str = serde_json::to_string_pretty(&request_body);
    log::info!("request_body \n {}", rb_str.unwrap());

    let parts = IndicesCreateParts::Index(index.name);
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
    pub name: &'a str,
    pub config: RequestBody,
}

// https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html
#[derive(Serialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FieldType {
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
    settings: Settings,
}

impl Default for RequestBody {
    fn default() -> Self {
        let analysis = Analysis {
            tokenizer: Tokenizer::SudachiTokenizer {
                r#type: "sudachi_tokenizer".to_owned(),
                split_mode: None,
            },
            analyzer: Analyzer::SudachiAnalyzer {
                filter: vec!["search".to_owned()],
                tokenizer: "sudachi_tokenizer".to_owned(),
                r#type: "custom".to_owned(),
            },
            filter: TokenFilter::Search {
                r#type: "sudachi_split".to_owned(),
                mode: "search".to_owned(),
            },
        };
        let cell = Cell {
            r#type: FieldType::Text,
            content: FieldType::Text,
        };
        let title = Property::Title {
            proeprties: cell.clone(),
        };
        let question = Property::Question {
            properties: cell.clone(),
        };
        let answer = Property::Answer { properties: cell };
        let tags = FieldType::Keyword;
            r#type: FieldType::Keyword,
        };

        let mappings = Mappings {
            properties: Properties {
                title,
                question,
                answer,
                tags,
            },
        };
        let settings = Settings { analysis };
        Self { mappings, settings }
    }
}

// Mapping parameters
// https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-params.html
#[derive(Serialize)]
pub struct Mappings {
    properties: Properties,
}

#[derive(Serialize)]
pub struct Properties {
    title: Property,
    question: Property,
    answer: Property,
    tags: FieldType,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Property {
    Title { proeprties: Cell },
    Question { properties: Cell },
    Answer { properties: Cell },
}

#[derive(Serialize, Clone)]
pub struct Cell {
    r#type: FieldType,
    content: FieldType,
}

// https://www.elastic.co/guide/en/elasticsearch/reference/current/index-modules.html#index-modules-settings
// https://docs.rs/elasticsearch/7.8.0-alpha.1/elasticsearch/indices/struct.IndicesExists.html
#[derive(Serialize)]
pub struct Settings {
    analysis: Analysis,
}

// https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-custom-analyzer.html
// https://www.elastic.co/guide/en/elasticsearch/reference/current/test-analyzer.html
#[derive(Serialize)]
pub struct Analysis {
    tokenizer: Tokenizer,
    analyzer: Analyzer,
    filter: TokenFilter,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tokenizer {
    SudachiTokenizer {
        r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        split_mode: Option<String>,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Analyzer {
    SudachiAnalyzer {
        filter: Vec<String>,
        tokenizer: String,
        r#type: String,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenFilter {
    Search {
        r#type: String,
        mode: String,
    },
    Synonym {
        r#type: String,
        synonyms: Vec<String>,
    },
    RomajiReadingForm {
        r#type: String,
        use_romaji: bool,
    },
    KatakanaReadingForm {
        r#type: String,
        use_romaji: bool,
    },
}
