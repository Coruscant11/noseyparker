use chrono::Duration;
use super::models;

// -------------------------------------------------------------------------------------------------
// Error
// -------------------------------------------------------------------------------------------------
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Reach 100 tries before receiving response for: {0}")]
    RateLimited(String),

    #[error("invalid base url: {0}")]
    UrlBaseError(url::Url),

    #[error("error parsing URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("error building URL: component {0:?} contains a slash")]
    UrlSlashError(String),

    #[error("error making request: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("error loading token: ill-formed value of {0} environment variable")]
    InvalidTokenEnvVar(String),
}
