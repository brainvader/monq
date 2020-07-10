use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseBody<T> {
    #[serde(alias = "_index")]
    index: String,
    #[serde(alias = "_type")]
    r#type: String,
    #[serde(alias = "_id")]
    id: String,
    #[serde(alias = "_version")]
    version: i64,
    #[serde(alias = "_seq_no")]
    seq_no: i64,
    #[serde(alias = "_primary_term")]
    primary_term: i64,
    found: bool,
    #[serde(alias = "_source")]
    pub source: T,
}
