mod cmd;

use clap::Parser;
use cmd::{command::App, Command};

fn main() {
    if let Err(e) = App::parse().execute() {
        println!("{}", e)
    }
}
