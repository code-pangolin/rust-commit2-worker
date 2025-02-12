#![feature(async_closure)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
pub mod api;
mod cmd;
pub mod rpc;
pub mod rpc_api;
mod storage;
// mod auth;
use std::env;

pub(crate) mod utils;

use clap::Parser;
use cmd::{command::App, Command};

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    fil_logger::init();

    if let Err(e) = App::parse().execute().await {
        println!("{:?}", e);
    }

    // get_params(params_json(), 2048).await;
    // get_srs(srs_json()).await;
}
