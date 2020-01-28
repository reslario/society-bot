use std::fmt::Display;
use serde::export::Formatter;

#[derive(Debug)]
pub enum Error {
    NoStatement,
    TweetFailed(Box<dyn std::error::Error>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,
               "Failed to tweet on {}, {}",
                chrono::Local::now().format("%Y-%m-%d at %H:%M:%S"),
                match self {
                    Error::NoStatement => "Failed to get statement to tweet".into(),
                    Error::TweetFailed(e) => format!("Failed to send tweet: {}", e)
                }
        )
    }
}