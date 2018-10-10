extern crate the_bob;
extern crate hyper;

use the_bob::credentials;
use the_bob::discord;

use hyper::rt;

fn main() {

    //TODO move this to discord::properties?
    let credentials = credentials::read_credentials_from_file("credentials");
    let bot_token = credentials.bot_token;

    rt::run(
        discord::get_gateway_information(bot_token)
    );
}