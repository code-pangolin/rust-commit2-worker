#![feature(async_closure)]
mod cmd;
use std::env;

pub(crate) mod utils;

use clap::Parser;
use cmd::{command::App, Command};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    fil_logger::init();

    if let Err(e) = App::parse().execute().await {
        println!("{}", e)
    }

    // get_params(params_json(), 2048).await;
    // get_srs(srs_json()).await;
}
