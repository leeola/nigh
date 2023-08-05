use anyhow::Result;
#[cfg(feature = "clap")]
use clap::Parser;
use unifi_protect::UnifiProtectServer;

pub mod client;

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
    server: UnifiProtectServer,
}
impl Nigh {
    pub fn new(config: Config) -> Self {
        let server = UnifiProtectServer::new(&config.host);
        Self { config, server }
    }
    pub async fn fetch_cameras(&mut self) -> Result<()> {
        self.server
            .login(&self.config.user, &self.config.pass)
            .await
            .map_err(|err: &str| anyhow::anyhow!(err.to_string()))?;
        self.server
            .fetch_cameras()
            .await
            .map_err(|err: String| anyhow::anyhow!(err))?;
        Ok(())
    }
}
