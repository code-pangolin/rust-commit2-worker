use async_trait::async_trait;
use clap::{command, Parser, Subcommand};
use once_cell::sync::Lazy;
use version::version;

use super::{run::cmd::Run, Command};

static DEFAULT_WORKER_REPO: Lazy<String> = Lazy::new(|| format!("{}/.lotusworker", home_dir()));

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "lotus-worker")]
#[command(bin_name = "lotus-worker")]
#[command(version = version!())]
#[command(about = "Remote miner worker", long_about = None)]
pub struct App {
    /// Specify worker repo path
    #[arg(
        long,
        env = "LOTUS_WORKER_PATH",
        default_value = DEFAULT_WORKER_REPO.as_str(),
        global = true
    )]
    worker_repo: String,

    #[command(subcommand)]
    command: AppSubcommands,
}

#[derive(Debug, Subcommand)]
enum AppSubcommands {
    Run(Run),
}

#[async_trait]
impl Command for App {
    async fn action(&self) -> anyhow::Result<()> {
        match &self.command {
            AppSubcommands::Run(command) => command.execute().await?,
        }

        Ok(())
    }
}

fn home_dir() -> String {
    home::home_dir()
        .unwrap()
        .into_os_string()
        .to_str()
        .unwrap()
        .to_string()
}
