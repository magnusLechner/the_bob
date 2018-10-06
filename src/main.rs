extern crate the_bob;

use the_bob::setting;
use the_bob::discord;

fn main() {

    let settings = setting::read_settings_from_file("credentials");

    println!("{:?}", settings);

    discord::run_get();
    discord::run_post();
//    discord::run_authentication(settings.bot_token);
    discord::authenticate_bot(settings.bot_token);
}