use structopt::StructOpt;
use rust_twitter_bot_lib::TwitterBot;

#[derive(StructOpt)]
pub struct Auth {
    #[structopt(flatten)]
    pub consumer: Consumer,
    #[structopt(flatten)]
    pub access: Access,
}

#[derive(StructOpt)]
pub struct Consumer {
    #[structopt(long = "consumer-key")]
    pub key: String,
    #[structopt(name = "consumer-secret", long)]
    pub secret: String
}

#[derive(StructOpt)]
pub struct Access {
    #[structopt(long = "access-token")]
    pub token: String,
    #[structopt(name = "access-secret", long)]
    pub secret: String
}

impl Auth {
    pub fn into_bot(self) -> TwitterBot {
        TwitterBot::new()
            .consumer_key(&self.consumer.key)
            .consumer_secret_key(&self.consumer.secret)
            .access_token(&self.access.token)
            .secret_access_token(&self.access.secret)
    }
}