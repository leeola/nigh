use anyhow::Result;
#[cfg(feature = "clap")]
use clap::Parser;
use nigh_protect_api::ProtectApi;

#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct Config {
    /// The protect API host
    #[cfg_attr(feature = "clap", arg(long, env = "PROTECT_API_HOST"))]
    host: String,
    #[cfg_attr(feature = "clap", arg(long, env = "PROTECT_API_USER"))]
    user: String,
    #[cfg_attr(feature = "clap", arg(long, env = "PROTECT_API_PASS"))]
    pass: String,
}
pub struct Nigh {
    config: Config,
    client: ProtectApi,
}
impl Nigh {
    pub fn new(config: Config) -> Result<Self> {
        let client = ProtectApi::new(config.host.as_str())?;
        Ok(Self { config, client })
    }
    pub async fn fetch_cameras(&mut self) -> Result<()> {
        // self.server
        //     .login(&self.config.user, &self.config.pass)
        //     .await
        //     .map_err(|err: &str| anyhow::anyhow!(err.to_string()))?;
        // self.server
        //     .fetch_cameras()
        //     .await
        //     .map_err(|err: String| anyhow::anyhow!(err))?;
        Ok(())
    }
}
