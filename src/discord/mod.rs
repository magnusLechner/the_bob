mod gateway;
mod properties;

use std::str;

use hyper::{header, Client, Method, Request, Body};
use hyper::rt::{Future, Stream};

use hyper_tls::HttpsConnector;

use serde_json::{self, Value, Error};

use self::gateway::opcode::{self, OpcodeValue};
use self::properties::DiscordProperties;

pub fn authenticate_bot(token: String) {

    let opcode = OpcodeValue::Identify;

    println!("opcode: {:?}", opcode::get_opcode_value(opcode));
}


//TODO request wrapper drum rum und die Header auslagern mit einer Methode,
//TODO welche ein Request entgegennimmt und dann alle Header autoamtische setzt
pub fn get_gateway_information(bot_token: &String) -> impl Future<Item=(), Error=()> {
    let discord_properties = get_discord_properties();
    let gateway_information_url = get_gateway_information_url(&discord_properties);

    let https = HttpsConnector::new(4).unwrap();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let req = Request::get(gateway_information_url)
        .header(header::CONTENT_TYPE, discord_properties.get_header_value("content_type"))
        .header(header::AUTHORIZATION, "Bot ".to_owned() + &bot_token)
        .header(header::USER_AGENT, discord_properties.get_header_value("user_agent"))
        .body(Body::empty()).unwrap();

    let response_stream = client.request(req).and_then(|res| {
        println!("POST RESPONSE STATUS: {}", res.status());

        res.into_body().concat2()
    });

    response_stream.map(|(get_chunk)| {
        let response_as_str = str::from_utf8(&get_chunk).unwrap();
        println!("POST RESPONSE BODY: {:?}", response_as_str);

        let gateway_information: GatewayInformation = serde_json::from_str(response_as_str).unwrap();
        println!("GATEWAY RESPONSE: {}", gateway_information.url);
        println!("GATEWAY RESPONSE: {}", gateway_information.session_start_limit.reset_after);

        gateway_url = gateway_information.url;

    })
    .map_err(|err| {
        println!("Error: {}", err);
    })


    //TODO websocket aufbauen
}

fn get_discord_properties() -> DiscordProperties {
    let discord_header_location = "src/resources/discord_header";
    let discord_api_location = "src/resources/discord_api";
    properties::load_discord_properties(discord_header_location, discord_api_location)
}

fn get_gateway_information_url(discord_properties: &DiscordProperties) -> String {
    let base_url = discord_properties.get_api_value("discord_api_base_url");
    let api_version = discord_properties.get_api_value("discord_api_version");
    let resource = discord_properties.get_api_value("discord_api_gateway_bot");

    let gateway_information = base_url + &api_version + &resource;
    gateway_information
}

#[derive(Serialize, Deserialize)]
struct GatewayInformation {
    url: String,
    shards: u8,
    session_start_limit: SessionStartLimit
}

#[derive(Serialize, Deserialize)]
struct SessionStartLimit {
    total: i32,
    remaining: i32,
    reset_after: i64
}