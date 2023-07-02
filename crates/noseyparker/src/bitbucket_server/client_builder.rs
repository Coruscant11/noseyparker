use reqwest::{IntoUrl, Url};
use tracing::debug;

use super::{Auth, Client, Error, Result};

pub struct ClientBuilder {
    base_url: reqwest::Url,
    auth: Auth
}

impl ClientBuilder {
    const USER_AGENT: &str = "noseyparker";

    pub fn new<T: IntoUrl>(url: T) -> Self {
        ClientBuilder {
            base_url: url.into_url().expect("Could not parse the given Bitbucket Server API base URL"),
            auth: Auth::Unauthenticated,
        }
    }

    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn personal_access_token_from_env(self) -> Result<Self> {
        self.personal_access_token_from_env_var("NP_BITBUCKET_SERVER_TOKEN")
    }

    fn personal_access_token_from_env_var(mut self, env_var_name: &str) -> Result<Self> {
        match std::env::var(env_var_name) {
            Err(std::env::VarError::NotPresent) => {
                debug!("No Bitbucket Server access token provided; using unauthenticated API access.");
            }
            Err(std::env::VarError::NotUnicode(_s)) => {
                return Err(Error::InvalidTokenEnvVar(env_var_name.to_string()));
            }
            Ok(val) => {
                debug!("Using Bitbucket Server access token from {env_var_name} environment variable");
                self.auth = Auth::PersonalAccessToken(secrecy::SecretString::from(val))
            }
        }
        Ok(self)
    }

    pub fn build(self) -> Result<Client> {
        let inner = reqwest::ClientBuilder::new()
            .user_agent(Self::USER_AGENT)
            .build()?;
        Ok(Client {
            base_url: self.base_url,
            auth: self.auth,
            inner
        })
    }
}