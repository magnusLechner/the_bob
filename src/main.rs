extern crate the_bob;

use the_bob::credentials;
use the_bob::discord;

//TODO remove
use the_bob::hyper_example;

fn main() {

    //TODO move this to discord::properties?
    let credentials = credentials::read_credentials_from_file("credentials");

    println!("{:?}", credentials);

//    hyper_example::run_get();
//    hyper_example::run_post();

    discord::authenticate_bot(credentials.bot_token);

    discord::get_gateway_information();
}