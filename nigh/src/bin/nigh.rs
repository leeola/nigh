use clap::{Parser, Subcommand};
use tracing::{metadata::LevelFilter, subscriber};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The protect API host
    #[arg(long, env = "PROTECT_API_HOST")]
    host: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
}

fn main() {
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

    let config = Cli::parse();
    dbg!(&config);

    println!("Hello, world!");
}
