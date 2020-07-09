use serde::Deserialize;

#[derive(Deserialize)]
pub struct Shard {
    pub failed: i64,
    pub successful: i64,
    pub total: i64,
}

#[derive(Deserialize)]
pub struct PostResponseBody {
    #[serde(alias = "_index")]
    pub index: String,
    #[serde(alias = "_type")]
    pub r#type: String,
    #[serde(alias = "_id")]
    pub id: String,
    #[serde(alias = "_version")]
    pub version: i64,
    pub result: String,
    #[serde(alias = "_shards")]
    pub shards: Shard,
    #[serde(alias = "_seq_no")]
    pub seq_no: i64,
    #[serde(alias = "_primary_term")]
    pub primary_term: i64,
}
