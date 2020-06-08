use dotenv;
use serenity::{framework::standard::StandardFramework, prelude::*};
use std::env;

mod event;
use event::Handler;

mod commands;
use commands::{emoji, general, EMOJI_GROUP, GENERAL_GROUP};

fn main() {
    dotenv::dotenv().expect("Could not load env file");
    let token = env::var("DISCORD_TOKEN").expect("There is not token");
    let mut client = Client::new(&token, Handler).expect("Could not start client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("*"))
            .help(&general::HELP)
            .help(&emoji::HELP)
            .group(&GENERAL_GROUP)
            .group(&EMOJI_GROUP),
    );

    if let Err(msg) = client.start() {
        println!("Error: {:?}", msg);
    }
}
