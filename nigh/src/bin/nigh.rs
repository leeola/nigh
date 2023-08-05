use anyhow::Result;
use clap::{Parser, Subcommand};
use nigh::{Config, Nigh};
use tracing::{metadata::LevelFilter, subscriber};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    config: Config,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Move logging init to a core utility, for ease of test setup.
    subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .with_env_var("RUST_LOG")
                    .from_env_lossy(),
            )
            .finish(),
    )
    .unwrap();

    let Cli { config, command } = Cli::parse();
    let mut nigh = Nigh::new(config).unwrap();
    match command {
        Commands::List => nigh.fetch_cameras().await?,
    }
    Ok(())
}
