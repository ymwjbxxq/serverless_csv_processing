use csv;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[serde(alias = "tconst")]
    pub id: String,

    #[serde(default = "default_label")]
    pub label: String,

    #[serde(alias = "titleType")]
    pub title_type: String,

    #[serde(alias = "primaryTitle")]
    pub primary_title: String,

    #[serde(alias = "originalTitle")]
    pub original_title: String,

    #[serde(alias = "isAdult")]
    #[serde(deserialize_with = "bool_from_string")]
    pub is_adult: bool,

    #[serde(alias = "startYear")]
    #[serde(deserialize_with = "csv::invalid_option")]
    pub start_year: Option<u16>,

    #[serde(alias = "endYear")]
    #[serde(deserialize_with = "csv::invalid_option")]
    pub end_year: Option<u16>,

    #[serde(alias = "runtimeMinutes")]
    #[serde(deserialize_with = "csv::invalid_option")]
    pub runtime_minutes: Option<u16>,

    #[serde(alias = "genres")]
    #[serde(deserialize_with = "csv::invalid_option")]
    pub genres: Option<String>,
}

fn default_label() -> String {
    "movies".to_string()
}

/// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Ok(false),
    }
}
