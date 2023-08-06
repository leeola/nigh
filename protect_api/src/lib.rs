use anyhow::anyhow;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::debug;
use url::{ParseError, Url};

pub mod api;

const HEADER_CSRF: &str = "X-CSRF-Token";
const HEADER_SET_COOKIE: &str = "Set-Cookie";

#[derive(Debug, Error)]
pub enum NewProtectError {
    #[error("protect url cannot be base. url:{0}")]
    ProtectUrlCannotBeBase(Url),
    #[error(transparent)]
    ParseBaseUrl(#[from] ParseError),
}
#[derive(Debug, Error)]
pub enum LoginError {
    #[error("protect url cannot be base. url:{0}")]
    ProtectUrlCannotBeBase(Url),
    #[error("login request failed: {0}")]
    RequestFailed(anyhow::Error),
    #[error("missing or invalid header: {header}")]
    MissingOrInvalidHeaderResponse { header: &'static str },
}

pub struct ProtectApi {
    base_url: Url,
    auth_headers: HeaderMap,
}
impl ProtectApi {
    pub fn new(
        protect_api_base_url: impl TryInto<Url, Error = ParseError>,
    ) -> Result<Self, NewProtectError> {
        let base_url = protect_api_base_url.try_into()?;
        // Fail early if the URL is unable to be combined.
        let _ = base_url
            .login_url()
            .map_err(NewProtectError::ProtectUrlCannotBeBase)?;
        Ok(Self {
            base_url,
            auth_headers: Default::default(),
        })
    }
    pub async fn login(
        &mut self,
        user: impl Into<String>,
        pass: impl Into<String>,
    ) -> Result<(), LoginError> {
        // NOTE: The reference impl fetches CRSF .. not sure if that's necessary? I'd imagine it's
        // just as effective as getting it once..?
        let url = self
            .base_url
            .login_url()
            .map_err(LoginError::ProtectUrlCannotBeBase)?;
        let user = user.into();
        let pass = pass.into();
        debug!(%url, user, pass="*".repeat(pass.chars().count()), "calling login api");
        let resp = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("reqwest client failed to build")
            .post(url)
            .json(&LoginRequest {
                username: user,
                password: pass,
                remember_me: true,
                token: "".into(),
            })
            .send()
            .await
            .map_err(|err| LoginError::RequestFailed(err.into()))?;
        if !resp.status().is_success() {
            // Not sure if any of these are actionable, yet.
            return Err(LoginError::RequestFailed(anyhow!(
                "unexpected status: {}",
                resp.status()
            )));
        }

        let csrf_token = resp.headers().get(HEADER_CSRF).cloned().ok_or(
            LoginError::MissingOrInvalidHeaderResponse {
                header: HEADER_CSRF,
            },
        )?;
        let cookie = resp.headers().get(HEADER_SET_COOKIE).cloned().ok_or(
            LoginError::MissingOrInvalidHeaderResponse {
                header: HEADER_SET_COOKIE,
            },
        )?;
        let mut map = HeaderMap::new();
        map.insert(HEADER_CSRF, csrf_token);
        map.insert(HEADER_SET_COOKIE, cookie);
        self.auth_headers = map;
        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
    pub token: String,
}
trait WithPathSegments: Sized + Clone {
    fn and_path_segments<I>(self, segments: I) -> Result<Self, Self>
    where
        I: IntoIterator,
        I::Item: AsRef<str>;
}
impl WithPathSegments for Url {
    fn and_path_segments<I>(mut self, segments: I) -> Result<Self, Self>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let ok = match self.path_segments_mut() {
            Ok(mut segs) => {
                segs.extend(segments);
                true
            },
            Err(()) => false,
        };
        if ok {
            Ok(self)
        } else {
            Err(self)
        }
    }
}
trait ProtectApiBaseUrl: Sized + Clone {
    fn login_url(&self) -> Result<Self, Self>;
}
impl ProtectApiBaseUrl for Url {
    fn login_url(&self) -> Result<Self, Self> {
        self.clone().and_path_segments(["api", "auth", "login"])
    }
}
