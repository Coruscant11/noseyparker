use url::Url;

mod auth;
mod client;
mod client_builder;
mod error;
mod models;
mod repo_enumerator;
mod result;

pub use auth::Auth;
pub use client::Client;
pub use client_builder::ClientBuilder;
pub use error::Error;
pub use repo_enumerator::{RepoEnumerator, RepoSpecifiers};
pub use result::Result;

use crate::progress::Progress;

pub fn enumerate_repo_urls(
    repo_specifiers: &RepoSpecifiers,
    bitbucket_server_api_url: Url,
    progress: Option<&mut Progress>,
) -> anyhow::Result<Vec<String>> {
    use anyhow::Context;

    let client = ClientBuilder::new(bitbucket_server_api_url)
        .personal_access_token_from_env()
        .context("Failed to get GitHub access token from environment")?
        .build()
        .context("Failed to initialize GitHub client")?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("Failed to initialize async runtime")?;

    let result = runtime.block_on(async {
        let repo_enumerator = RepoEnumerator::new(&client);
        let repo_urls = repo_enumerator
            .enumerate_repo_urls(repo_specifiers, progress)
            .await?;
        Ok(repo_urls)
    });

    result
}
