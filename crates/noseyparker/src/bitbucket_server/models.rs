use serde::Deserialize;
use serde_json;

pub mod page;
pub use page::Page;
pub use page::Links;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientError {
    pub errors: Option<Vec<Error>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub message: String,
    pub context: String,
    pub exception_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    pub key: String,
    pub id: i64,
    #[serde(rename = "type")]
    pub project_type: String,
    pub public: bool,
    #[serde(default)]
    pub scope: String,
    pub description: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub avatar: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub name: String,
    pub id: i64,
    pub state: String,
    pub public: bool,
    pub scm_id: String,
    pub slug: String,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub description: String,
    pub project: Project,
    pub links: Links,
    #[serde(default)]
    pub related_links: serde_json::Value,
    #[serde(default)]
    pub partition: i64,
    pub hierarchy_id: String,
    pub status_message: String,
    pub archived: bool,
    pub forkable: bool,
    #[serde(default)]
    pub default_branch: bool,
    #[serde(default)]
    pub origin: serde_json::Value,
}