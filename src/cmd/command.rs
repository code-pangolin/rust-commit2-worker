use std::{ffi::OsString, panic::catch_unwind};

use clap::{command, Parser, Subcommand};
use once_cell::sync::Lazy;

use super::{run::Run, Command};

static DEFAULT_WORKER_REPO: Lazy<String> = Lazy::new(|| format!("{}/.lotusworker", home_dir()));

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "lotus-worker")]
#[command(bin_name = "lotus-worker")]
#[command(version = "1.0.0 beta")] //TODO: version
#[command(about = "Remote miner worker", long_about = None)]
pub struct App {
    /// Specify worker repo path
    #[arg(
        long,
        env = "LOTUS_WORKER_PATH",
        default_value = DEFAULT_WORKER_REPO.as_str(),
        global = true
    )]
    worker_repo: Option<OsString>,

    #[command(subcommand)]
    command: AppSubcommands,
}

#[derive(Debug, Subcommand)]
enum AppSubcommands {
    Run(Run),
}

impl Command for App {
    fn action(&self) -> anyhow::Result<()> {
        match &self.command {
            AppSubcommands::Run(command) => command.execute()?,
        }

        Ok(())
    }

    fn execute(&self) -> anyhow::Result<()> {
        match catch_unwind(|| {
            self.before()?;
            self.action()?;
            self.after()
        }) {
            Ok(e) => {
                return e;
            }
            Err(e) => {
                //TODO: generate panic reports
                panic!("{:?}", e)
            }
        };
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
