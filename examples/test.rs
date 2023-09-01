use std::os;

use anyhow::{Context, Result};
use chrono::Local;
use clap::{command, Parser};
use colored::Colorize;
use fvm_shared::address::Address;
use tokio::fs;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "test")]
#[command(bin_name = "test")]
#[command(about = "Remote miner worker", long_about = None)]
pub struct App {
    /// commit1out file path
    #[arg(long, short, default_value = "commit1out")]
    file: String,
    /// commit1out file path
    #[arg(long)]
    sector: u64,
    /// commit1out file path
    #[arg(long, default_value = "6")]
    sector_id: u64,
}

impl App {
    async fn execute(&self) -> Result<()> {
        let file = fs::read(&self.file).await.context("read commit1")?;
        let sector = self.sector.clone();

        let scp1o = serde_json::from_slice(&file)?;

        let maddr = Address::new_id(sector);

        let mut prover_id: [u8; 32] = [0; 32];
        let payload = maddr.payload().to_bytes();
        prover_id[..payload.len()].copy_from_slice(&payload);

        let start_time = Local::now();
        println!(
            "{} {}",
            format!("{}", start_time).blue(),
            "started seal_commit2".green()
        );
        let result = filecoin_proofs_api::seal::seal_commit_phase2(
            scp1o,
            prover_id,
            filecoin_proofs_api::SectorId::from(self.sector_id),
        )
        .context("seal_commit_phase2")?;
        let end_time = Local::now();
        println!(
            "{} {}",
            format!("{}", end_time).blue(),
            "end seal_commit2".green()
        );
        let time_difference = end_time.signed_duration_since(start_time);
        println!("{}{}", "time_took:".green(), time_difference);

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    println!("starting ...");
    let app = App::parse().execute().await;
    match app {
        Ok(_) => todo!(),
        Err(e) => {
            println!("error::::");
            println!("  {:#?}", e)
        }
    }
}
