use std::error::Error;
use std::fmt::{Display, Formatter};

pub type WolgramResult = Result<(), WolgramError>;

#[derive(Debug)]
pub enum WolgramError {
    ConfigError(confy::ConfyError),
    TelegramBotError(telegram_bot::Error),
    WakeyError(std::io::Error),
}

impl Error for WolgramError {}

impl Display for WolgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WolgramError::ConfigError(err) => {
                write!(
                    f,
                    "Wolgram has encountered an error! Failed to load configuration: {}",
                    err
                )
            }
            WolgramError::TelegramBotError(err) => {
                write!(
                    f,
                    "Wolgram has encountered an error! Telegram bot error: {}",
                    err
                )
            }
            WolgramError::WakeyError(err) => {
                write!(f, "Wolgram has encountered an error! WOL error: {}", err)
            }
        }
    }
}
