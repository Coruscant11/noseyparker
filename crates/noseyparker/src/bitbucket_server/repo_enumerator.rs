use super::models::Repository;
use super::{Client, Result};

use crate::progress::Progress;

/// A `RepoEnumerator` provides higher-level functionality on top of the GitHub REST API to list
/// repositories belonging to specific users or organizations.
pub struct RepoEnumerator<'c> {
    client: &'c Client,
}

impl<'c> RepoEnumerator<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Enumerate the accessible repositories that belong to the given user.
    pub async fn enumerate_project_repos(&self, project_key: &str) -> Result<Vec<Repository>> {
        let repo_page = self.client.get_project_repos(project_key).await?;
        self.client.get_all(repo_page, &["projects", project_key, "repos"]).await
    }

    /// Enumerate the repository clone URLs found from the according to the given `RepoSpecifiers`,
    /// collecting the union of specified repository URLs.
    ///
    /// The resulting URLs are sorted and deduplicated.
    pub async fn enumerate_repo_urls(
        &self,
        repo_specifiers: &RepoSpecifiers,
        mut progress: Option<&mut Progress>,
    ) -> Result<Vec<String>> {
        let mut repo_urls = Vec::new();

        for project in &repo_specifiers.project {
            let to_add = self.enumerate_project_repos(project).await?;
            if let Some(progress) = progress.as_mut() {
                progress.inc(to_add.len() as u64);
            }
            for r in to_add.into_iter() {
                for c in r.links.clone.into_iter() {
                    if c.name == "http" {
                        repo_urls.push(c.href)
                    }
                }
            }
        }

        repo_urls.sort();
        repo_urls.dedup();

        Ok(repo_urls)
    }
}

#[derive(Debug)]
pub struct RepoSpecifiers {
    pub project: Vec<String>,
}

impl RepoSpecifiers {
    pub fn is_empty(&self) -> bool {
        self.project.is_empty()
    }
}
