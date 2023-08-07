use crate::{ProtectApi, ProtectError};
use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, enabled, Level};
use url::Url;

impl ProtectApi {
    pub async fn list_cameras(&self) -> Result<Vec<Camera>, ProtectError> {
        let url: Url = self
            .base_url
            .clone()
            .and_path_segments(["proxy", "protect", "api", "cameras"])
            .into();
        debug!(%url, "calling cameras api");
        let resp = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("reqwest client failed to build")
            .get(url.clone())
            .headers(self.auth_headers.clone())
            .send()
            .await
            .map_err(|err| ProtectError::RequestFailed {
                url: url.clone(),
                err: err.into(),
            })?;
        let status = resp.status();
        if !status.is_success() {
            // Not sure if any of these are actionable, yet.
            return Err(ProtectError::RequestFailed {
                url,
                err: anyhow!("unexpected status: {status}"),
            });
        }

        let cameras = resp.json::<Vec<Camera>>().await.map_err(|err| {
            ProtectError::UnexpectedResponseJson {
                url,
                err: err.into(),
            }
        })?;
        Ok(cameras)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub mac: String,
    pub host: String,
    pub name: String,
}
