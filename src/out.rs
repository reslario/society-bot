use rust_twitter_bot_lib::TwitterBot;

const CONSUMER_KEY: &str = include_str!(r"..\keys\consumer.txt");
const CONSUMER_SECRET: &str = include_str!(r"..\keys\consumer_secret.txt");
const ACCESS_TOKEN: &str = include_str!(r"..\keys\access.txt");
const ACCESS_SECRET: &str = include_str!(r"..\keys\access_secret.txt");

pub fn format(stmt: String) -> String {
    format!("We live in a society where {}...", stmt)
}

pub fn print(sentence: String) {
    println!("{}", sentence)
}

pub fn tweet(sentence: String) {
    let auth = TwitterBot::new()
        .consumer_key(CONSUMER_KEY)
        .consumer_secret_key(CONSUMER_SECRET)
        .access_token(ACCESS_TOKEN)
        .secret_access_token(ACCESS_SECRET);
    auth.tweet(&sentence, None).unwrap();
}