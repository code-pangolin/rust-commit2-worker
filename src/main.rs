mod cmd;
mod lotus_log;
pub(crate) mod utils;

use fil_proofs_param::{get_params, get_srs, params_json, srs_json};

use lotus_log::LotusLog;
use once_cell::sync::Lazy;

static LOGGER: Lazy<LotusLog> = Lazy::new(|| { //TODO: https://github.com/filecoin-project/rust-fil-logger/tree/master
    let mut logger = LotusLog::new("lotus-worker".to_string(), log::Level::Trace);
    logger.set_time_format("%Y-%m-%dT%H:%M:%S%.3f%z".to_string());
    logger
});

#[tokio::main]
async fn main() {
    if let Err(e) = LOGGER.init() {
        panic!("set logger: {}", e)
    };

    // if let Err(e) = App::parse().execute() {
    //     println!("{}", e)
    // }

    get_params(params_json(), 2048).await;
    get_srs(srs_json()).await;
}
