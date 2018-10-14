mod gateway;
mod properties;

use std::str;

use tokio::prelude::Future;
use tokio::runtime::Runtime;

use hyper::{header, Client, Request, Body};
use hyper::client::HttpConnector;
use hyper::rt::Stream;

use hyper_tls::HttpsConnector;

use serde_json::{self, Value, Error};

use self::gateway::opcode::{self, OpcodeValue};
use self::properties::DiscordProperties;

pub struct Discord {
    bot_token: String,
    properties: DiscordProperties,
    client: hyper::Client<HttpsConnector<HttpConnector>, Body>,
    runtime: Runtime
}

impl Discord {
    pub fn new(bot_token: &str) -> Self {
        let token = bot_token.to_string();

        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        let properties = properties::load();

        let runtime = Runtime::new().unwrap();

        Discord {
            bot_token: token,
            properties,
            client,
            runtime
        }
    }

    pub fn connect(&mut self) {
        let websocket_connection_properties = Self::ask_for_websocket_connection_properties(&mut self);
        Self::build_up_websocket_connection(&self, websocket_connection_properties);
    }

    fn ask_for_websocket_connection_properties(&mut self) -> WebsocketConnectionProperties {
        let uri = build_uri(&self.properties);

        //TODO make initial call to discord api and get uri to build websocket
        //TODO make common interface for all calls (abstract and type-safe; see serenity)

        let req = Request::get(uri)
            .header(header::CONTENT_TYPE, self.properties.get_header_value("content_type"))
            .header(header::AUTHORIZATION, "Bot ".to_owned() + &self.bot_token)
            .header(header::USER_AGENT, self.properties.get_header_value("user_agent"))
            .body(Body::empty()).unwrap();


        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        let response_stream = client.request(req).and_then(|res| {
            println!("POST RESPONSE STATUS: {}", res.status());

            res.into_body().concat2()
        });

        let future = response_stream.map(|get_chunk| {
            let response_as_str = str::from_utf8(&get_chunk).unwrap();
            println!("POST RESPONSE BODY: {:?}", response_as_str);

            let gateway_information: WebsocketConnectionProperties = serde_json::from_str(response_as_str).unwrap();
            println!("GATEWAY RESPONSE: {}", gateway_information.url);
            println!("GATEWAY RESPONSE: {}", gateway_information.session_start_limit.reset_after);
        })
        .map_err(|err| {
            println!("Error: {}", err);
        });


        let answer = self.runtime.spawn(future);


        let ssl = SessionStartLimit {
            total: 0,
            remaining: 0,
            reset_after: 0
        };

        let ssl2 = WebsocketConnectionProperties {
            url: "".to_string(),
            shards: 0,
            session_start_limit: ssl
        };

        ssl2

    }

    fn build_up_websocket_connection(&self, gateway_information: WebsocketConnectionProperties) {
        //TODO create and use websocket
    }
}

//TODO remove -> see methods above
pub fn get_gateway_information(bot_token: &String) -> impl Future<Item=(), Error=()> {
    let discord_properties = properties::load();
    let gateway_information_url = build_uri(&discord_properties);

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

    response_stream.map(|get_chunk| {
        let response_as_str = str::from_utf8(&get_chunk).unwrap();
        println!("POST RESPONSE BODY: {:?}", response_as_str);

        let gateway_information: WebsocketConnectionProperties = serde_json::from_str(response_as_str).unwrap();
        println!("GATEWAY RESPONSE: {}", gateway_information.url);
        println!("GATEWAY RESPONSE: {}", gateway_information.session_start_limit.reset_after);
    })
    .map_err(|err| {
        println!("Error: {}", err);
    })

    //TODO websocket aufbauen
}

fn build_uri(discord_properties: &DiscordProperties) -> String {
    let base_url = discord_properties.get_api_value("discord_api_base_url");
    let api_version = discord_properties.get_api_value("discord_api_version");
    let resource = discord_properties.get_api_value("discord_api_gateway_bot");

    let uri = base_url + &api_version + &resource;
    uri
}

#[derive(Serialize, Deserialize)]
struct WebsocketConnectionProperties {
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