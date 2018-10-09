mod gateway;
mod properties;

use std::str;

use hyper::{Client, Method, Request, Body};
use hyper::header::HeaderValue;
use hyper::rt::{Future, Stream};

use self::gateway::opcode::{self, OpcodeValue};

pub fn authenticate_bot(token: String) {

    let opcode = OpcodeValue::Identify;

    println!("opcode: {:?}", opcode::get_opcode_value(opcode));
}

pub fn get_gateway_information() -> impl Future<Item=(), Error=()> {
    let gateway_information_url = get_gateway_information_url();

    let client = Client::new();

    let req = Request::get(gateway_information_url)
        .header("Content-Type", "application/json")
        .header("Authorization", "myToken")
        .header("User-Agent", "TheBob/0.1")
        .body(Body::empty()).unwrap();

    let get = client.request(req).and_then(|res| {
        println!("POST RESPONSE STATUS: {}", res.status());

        res.into_body().concat2()
    });

    get.map(|(get_chunk)| {
            println!("POST RESPONSE BODY: {:?}", str::from_utf8(&get_chunk).unwrap());
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
}

fn get_gateway_information_url() -> String {
    let discord_header_location = "src/resources/discord_header";
    let discord_api_location = "src/resources/discord_api";
    let discord_properties = properties::load_discord_properties(discord_header_location, discord_api_location);

    let base_url = discord_properties.api.get_str("discord_api_base_url").unwrap();
    let api_version = discord_properties.api.get_str("discord_api_version").unwrap();
    let resource = discord_properties.api.get_str("discord_api_gateway_bot").unwrap();

    let gateway_information = base_url + &api_version + &resource;
    gateway_information
}