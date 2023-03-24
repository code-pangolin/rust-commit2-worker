use std::{ffi::OsString, panic::catch_unwind};

use clap::{command, Parser, Subcommand};
use lazy_static::lazy_static;

use super::{run::Run, Command};

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

lazy_static! {
    static ref DEFAULT_WORKER_REPO: String = format!("{}/.lotusworker", home_dir());
    static ref DEFAULT_MINER_REPO: String = format!("{}/.lotusminer", home_dir());
}
