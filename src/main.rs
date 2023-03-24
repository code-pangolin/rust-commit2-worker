mod cmd;
mod lotus_log;

use clap::Parser;

use cmd::{command::App, Command};

use lotus_log::LotusLog;
use once_cell::sync::Lazy;

static LOGGER: Lazy<LotusLog> = Lazy::new(|| {
    let mut logger = LotusLog::new("lotus-worker".to_string(), log::Level::Info);
    logger.set_time_format("%Y-%m-%dT%H:%M:%S%.3f%z".to_string());
    logger
});

fn main() {
    if let Err(e) = LOGGER.init() {
        panic!("set logger: {}", e)
    };

    if let Err(e) = App::parse().execute() {
        println!("{}", e)
    }
}
