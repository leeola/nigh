use base_url::{BaseUrl, BaseUrlError};
use reqwest::header::HeaderMap;
use thiserror::Error;
use url::{ParseError, Url};

pub mod api;
pub mod base_url;

#[derive(Debug, Error)]
pub enum NewProtectError {
    #[error("protect url cannot be base. url:{0}")]
    ProtectUrlCannotBeBase(Url),
    #[error(transparent)]
    ParseBaseUrl(#[from] ParseError),
}
impl From<BaseUrlError> for NewProtectError {
    fn from(err: BaseUrlError) -> Self {
        match err {
            BaseUrlError::CannotBeBase(url) => Self::ProtectUrlCannotBeBase(url),
            BaseUrlError::ParseBaseUrl(err) => Self::ParseBaseUrl(err),
        }
    }
}
/// A general set of protect errors for API calls that don't have actionable error variants.
#[derive(Debug, Error)]
pub enum ProtectError {
    #[error("request {url} failed: {err}")]
    RequestFailed { url: Url, err: anyhow::Error },
    #[error("request {url} failed: {err}")]
    UnexpectedResponseJson { url: Url, err: anyhow::Error },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct ProtectApi {
    base_url: BaseUrl,
    auth_headers: HeaderMap,
}
impl ProtectApi {
    pub fn new(
        protect_api_base_url: impl TryInto<BaseUrl, Error = BaseUrlError>,
    ) -> Result<Self, NewProtectError> {
        let base_url = protect_api_base_url.try_into()?;
        Ok(Self {
            base_url,
            auth_headers: Default::default(),
        })
    }
}
