use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::{Context, EventHandler},
};
use std::env;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, rdy: Ready) {
        const ENV_BOT_NAME: &str = "BOT_NAME";
        env::set_var(ENV_BOT_NAME, rdy.user.tag());
        println!(
            "{} is ready",
            env::var(ENV_BOT_NAME).expect("Bot name was not set up as an env variable")
        );
    }

    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "?ping" {
            if let Err(err) = msg.channel_id.say(&ctx.http, "Pong?") {
                println!("Error giving message: {:?}", err);
            }
        }
    }
}
