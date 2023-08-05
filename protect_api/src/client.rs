use thiserror::Error;
use url::{ParseError, Url};

#[derive(Debug, Error)]
pub enum NewClientError {
    #[error(transparent)]
    ParseBaseUrl(#[from] ParseError),
}

pub struct Client {
    base_url: Url,
}
impl Client {
    pub fn new(
        protect_api_base_url: impl TryInto<Url, Error = ParseError>,
    ) -> Result<Self, NewClientError> {
        let base_url = protect_api_base_url.try_into()?;
        Ok(Self { base_url })
    }
}
