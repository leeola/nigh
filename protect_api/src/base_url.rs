use std::{convert::Infallible, fmt, ops::Deref};
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Debug, Error)]
pub enum BaseUrlError {
    #[error(transparent)]
    ParseBaseUrl(#[from] ParseError),
    #[error("cannot construct BaseUrl from Url that can't be base")]
    CannotBeBase(Url),
}
impl From<Infallible> for BaseUrlError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
/// A validation for a `Url` where it is `!cannot_be_base()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseUrl(Url);
impl BaseUrl {
    pub fn new<U>(url: U) -> Result<Self, BaseUrlError>
    where
        Url: TryFrom<U>,
        BaseUrlError: From<<Url as TryFrom<U>>::Error>,
    {
        let url: Url = url.try_into()?;
        if url.cannot_be_a_base() {
            Err(BaseUrlError::CannotBeBase(url))
        } else {
            Ok(Self(url))
        }
    }
    pub fn and_path_segments<I>(mut self, segments: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        {
            let mut segs = self
                .0
                .path_segments_mut()
                .expect("BaseUrl enforces !cannot_be_base");
            segs.extend(segments);
        }
        self
    }
}
impl TryFrom<&'_ str> for BaseUrl {
    type Error = BaseUrlError;
    fn try_from(url: &'_ str) -> Result<Self, Self::Error> {
        Self::new(url)
    }
}
impl TryFrom<Url> for BaseUrl {
    type Error = BaseUrlError;
    fn try_from(url: Url) -> Result<Self, Self::Error> {
        Self::new(url)
    }
}
impl From<BaseUrl> for Url {
    fn from(BaseUrl(url): BaseUrl) -> Self {
        url
    }
}
impl Deref for BaseUrl {
    type Target = Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl fmt::Display for BaseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
