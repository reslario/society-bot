use rust_twitter_bot_lib::TwitterBot;
use structopt::StructOpt;
use std::time::Duration;

mod wikipedia;
mod qa;
mod get;
mod out;
mod auth;
mod err;

fn main() {
    let bot = auth::Auth::from_args()
        .into_bot();

    loop {
        tweet(&bot);
        std::thread::sleep(Duration::from_secs(60 * 60))
    }
}

#[allow(unused_must_use)]
fn tweet(bot: &TwitterBot) {
    get::get_statement()
        .map(out::format)
        .ok_or(err::Error::NoStatement)
        .and_then(|sentence| out::tweet(sentence, &bot)
            .map_err(err::Error::TweetFailed)
        ).map_err(|e| println!("{}", e));
}
