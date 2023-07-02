use std::{thread, time};
use anyhow::bail;
use chrono::{DateTime, Duration, TimeZone, Utc};
use reqwest;
use reqwest::{header, header::HeaderValue, StatusCode, Url, IntoUrl};
use secrecy::ExposeSecret;

use super::models::{Repository, Project, Page};
use super::{Auth, ClientBuilder, Error, Result};

pub struct Client {
    pub(super) base_url: Url,
    pub(super) inner: reqwest::Client,
    pub(super) auth: Auth,
}

const MAX_PER_PAGE: (&str, &str) = ("limit", "100");

impl Client {
    pub fn new<T: IntoUrl>(base_url: T) -> Result<Self> {
        ClientBuilder::new(base_url).build()
    }

    pub fn is_authenticated(&self) -> bool {
        match self.auth {
            Auth::Unauthenticated => false,
            Auth::PersonalAccessToken(_) => true
        }
    }

    pub async fn get_project_repos(&self, project_key: &str) -> Result<Page<Repository>> {
        let response = self
            .get_with_params(&["projects", project_key, "repos"], &[MAX_PER_PAGE])
            .await?;
        Ok(response.json().await?)
    }

    async fn next_page_inner<T>(&self, next: Option<i64>, path_parts: &[&str]) -> Result<Option<Page<T>>>
        where
            T: serde::de::DeserializeOwned,
    {
        match next {
            Some(next) => {
                let response = self.get_with_params(path_parts, &[MAX_PER_PAGE, ("start", next.to_string().as_str())]).await?;
                Ok(Some(response.json().await?))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all<T>(&self, page: Page<T>, path_parts: &[&str]) -> Result<Vec<T>>
        where
            T: serde::de::DeserializeOwned,
    {
        let mut results = Vec::new();
        let mut next_page = Some(page);
        while let Some(page) = next_page {
            results.extend(page.values.into_iter());
            next_page = self.next_page_inner(page.next_page_start, path_parts).await?;
        }
        Ok(results)
    }
}

impl Client {
    fn make_url(&self, path_parts: &[&str], params: &[(&str, &str)]) -> Result<Url> {
        url_from_path_parts_and_params(self.base_url.clone(), path_parts, params)
    }

    async fn get(&self, path_parts: &[&str]) -> Result<reqwest::Response> {
        self.get_with_params(path_parts, &[]).await
    }

    async fn get_with_params(
        &self,
        path_parts: &[&str],
        params: &[(&str, &str)],
    ) -> Result<reqwest::Response> {
        let url = self.make_url(path_parts, params)?;
        self.get_url(url).await
    }

    async fn get_url(&self, url: Url) -> Result<reqwest::Response> {
        // build request, handling authentication if any
        let mut tries = 0;
        while tries < 100 {
            tries += 1;
            let request_builder = self
                .inner
                .get(url.clone())
                .header(header::ACCEPT, "application/json");

            let request_builder = match &self.auth {
                Auth::PersonalAccessToken(token) => request_builder.bearer_auth(token.expose_secret()),
                Auth::Unauthenticated => request_builder,
            };
            let response = request_builder.send().await?;
            if response.status() == StatusCode::TOO_MANY_REQUESTS {
                continue
            }
            else {
                let response = response.error_for_status()?;
                return Ok(response);
            }
        }

        return Err(Error::RateLimited(url.to_string()) )
    }
}

/// Create a URL from the given base, path parts, and parameters.
///
/// The path parts should not contain slashes.
fn url_from_path_parts_and_params(
    base_url: Url,
    path_parts: &[&str],
    params: &[(&str, &str)],
) -> Result<Url> {
    if base_url.cannot_be_a_base() {
        return Err(Error::UrlBaseError(base_url));
    }

    let mut buf = base_url.path().to_string();
    if !buf.ends_with('/') {
        buf.push('/');
    }

    for (i, p) in path_parts.iter().enumerate() {
        if p.contains('/') {
            return Err(Error::UrlSlashError(p.to_string()));
        }
        if i > 0 {
            // do not add a leading slash for the very first path part, or the result comes out
            // wrong, as it is unintentionally treated as an absolute path
            //
            // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c2674663bf5e681b5bdb302d1b050237
            buf.push('/');
        }
        buf.push_str(p);
    }
    let url = base_url.join(&buf)?;
    let url = if params.is_empty() {
        Url::parse(url.as_str())
    } else {
        Url::parse_with_params(url.as_str(), params)
    }?;
    Ok(url)
}