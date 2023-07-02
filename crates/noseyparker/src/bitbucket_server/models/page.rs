use url::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub size: i64,
    pub limit: i64,
    pub is_last_page: bool,
    pub start: i64,
    #[serde(default)]
    pub next_page_start: Option<i64>,
    pub values: Vec<T>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloneLink {
    pub href: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfLink {
    pub href: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub clone: Vec<CloneLink>,
    #[serde(rename = "self")]
    pub self_link: Vec<SelfLink>,
}