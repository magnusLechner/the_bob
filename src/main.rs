extern crate the_bob;
extern crate hyper;
extern crate tokio;

use std::env;

use the_bob::credentials;
use the_bob::discord;

use hyper::rt;

use tokio::prelude::Future;
use tokio::runtime::Runtime;

fn main() {
    let discord_bot_token = env::var("DISCORD_BOT_TOKEN").unwrap();
    let mut tokioRuntime = Runtime::new().unwrap();
    let future = discord::get_gateway_information(&discord_bot_token);
    tokioRuntime.spawn(future);
    tokioRuntime.shutdown_on_idle().wait().unwrap();
}