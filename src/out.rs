use rust_twitter_bot_lib::TwitterBot;
use std::error::Error;

pub fn format(stmt: String) -> String {
    format!("We live in a society where {}...", stmt)
}

pub fn print(sentence: String) {
    println!("{}", sentence)
}

pub fn tweet(sentence: String, bot: &TwitterBot) -> Result<(), Box<dyn Error>> {
    bot.tweet(&sentence, None)
        .map(drop)
}