extern crate futures;
extern crate telegram_bot;
extern crate wakey;

use crate::bot::start_bot_loop;
use crate::config::WolgramConfig;
use crate::error::WolgramError;
use carlog::*;
use std::process::exit;

mod bot;
mod config;
mod error;
mod wol;

#[tokio::main]
async fn main() -> Result<(), telegram_bot::Error> {
    let cfg = match confy::load::<WolgramConfig>("wolgram") {
        Ok(val) => val,
        Err(err) => {
            carlog_error!(format!("{}", WolgramError::ConfigError(err)));
            exit(1);
        }
    };
    if let Err(err) = start_bot_loop(cfg).await {
        carlog_error!(format!("{}", err));
        exit(1);
    }
    Ok(())
}
