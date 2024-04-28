use std::sync::Arc;

pub mod types;
pub mod util;

use lazy_static::lazy_static;
use thiserror::Error;
use types::{
    ApplicationStatistics, AuthEndpoint, College, ScattergramSource, ScattergramSources,
    SchoolsImThinkingAbout,
};
use url::Url;
use uuid::Uuid;

use crate::types::{Config, Endpoint, PublicEndpoint};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Client {
    // Already wrapped in an Arc
    http_client: reqwest::Client,
    inner:       Arc<ClientRef>,
}

#[derive(Debug)]
struct ClientRef {
    key:      String,
    api_base: Url,
}

lazy_static! {
    static ref STUDENT_BASE: Url = Url::parse("https://student.naviance.com/").unwrap();
}
impl Client {
    /// Create a new client from an API key with sensible HTTP client settings
    pub async fn new(key: String) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .https_only(true)
            .use_rustls_tls()
            .brotli(true)
            .gzip(true)
            .zstd(true)
            .deflate(true)
            .build()?;
        Self::new_with_client(key, http_client).await
    }

    /// Create a new client from an API key with a custom HTTP client
    ///
    /// Warning: You should probably use an HTTP client with HTTPS-only
    pub async fn new_with_client(key: String, http_client: reqwest::Client) -> Result<Self> {
        let student_base = STUDENT_BASE.clone();
        let config_resp = Config::request(student_base, &http_client).await?;
        let config = Config::extract(config_resp).await?;
        let api_base_raw = config.api_host.ok_or(Error::Other(
            "No API host found in rewritten_config.js".to_string(),
        ))?;
        let api_base = Url::parse(&api_base_raw)?;
        let inner = Arc::new(ClientRef { key, api_base });
        Ok(Self { http_client, inner })
    }

    /// Fetches all the schools on your personal list
    pub async fn get_schools_im_thinking_about(&self) -> Result<types::SchoolsImThinkingAbout> {
        self.fetch_auth::<SchoolsImThinkingAbout>().await
    }

    /// Fetches all the possible scattergram sources for your school
    pub async fn get_scattergram_sources(&self) -> Result<ScattergramSources> {
        self.fetch_auth::<ScattergramSources>().await
    }

    /// Fetches a specific college's information given its UUID
    pub async fn get_college_info_by_uuid(&self, uuid: &Uuid) -> Result<College> {
        let base = self.api_base().clone();
        let resp = College::request(base, &self.http_client, self.key(), uuid).await?;
        College::extract(resp).await
    }

    pub async fn get_application_stats_by_uuid(
        &self,
        uuid: &Uuid,
    ) -> Result<ApplicationStatistics> {
        let base = self.api_base().clone();
        let resp =
            ApplicationStatistics::request(base, &self.http_client, self.key(), uuid).await?;
        ApplicationStatistics::extract(resp).await
    }

    /// Fetches any static public endpoint - use with caution
    pub async fn fetch<T: PublicEndpoint>(&self) -> Result<T::Response> {
        let base = self.api_base().clone();
        let resp = T::request(base, &self.http_client).await?;
        T::extract(resp).await
    }

    /// Fetches any static authenticated endpoint - use with caution
    pub async fn fetch_auth<T: AuthEndpoint>(&self) -> Result<T::Response> {
        let base = self.api_base().clone();
        let resp = T::request(base, &self.http_client, self.key()).await?;
        T::extract(resp).await
    }

    pub fn key(&self) -> &str { &self.inner.key }

    pub fn api_base(&self) -> &Url { &self.inner.api_base }
}
