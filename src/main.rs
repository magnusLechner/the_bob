extern crate the_bob;

use std::env;

use the_bob::discord::Discord;

fn main() {
    let bot_token = env::var("DISCORD_BOT_TOKEN").unwrap();
    let discord = Discord::new(bot_token.as_str());
    discord.connect();
}