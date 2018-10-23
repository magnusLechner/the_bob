mod gateway;

use std::str;

use tokio;

use hyper::{header, Client, Request, Body};
use hyper::client::HttpConnector;
use hyper::rt::{Future, Stream};

use hyper_tls::HttpsConnector;

use serde_json;

use self::gateway::{Opcode, Payload};

pub struct Discord {
    bot_token: String
}

impl Discord {
    pub fn new(bot_token: &str) -> Self {
        Discord {
            bot_token: bot_token.to_string()
        }
    }

    pub fn connect(&self) {
        Self::setup_websocket_connection(self);
    }

    fn setup_websocket_connection(&self) {
        let websocket_connection_properties = Self::get_websocket_connection_properties(self);
        Self::establish_connection(websocket_connection_properties);
    }

    //TODO establish connection
    fn establish_connection(websocket_connection_properties: WebsocketConnectionProperties) {
        let connection_url = websocket_connection_properties.get_explicit_websocket_url();

    }

    fn get_websocket_connection_properties(&self) -> WebsocketConnectionProperties {
        let client = Self::get_client();
        let req = Self::build_request(self);

        let response_stream = client.request(req).and_then(|res| {
            res.into_body().concat2()
        });

        let future = response_stream.map(|get_chunk| -> WebsocketConnectionProperties {
            let response_as_str = str::from_utf8(&get_chunk).unwrap();
            let websocket_connection_properties: WebsocketConnectionProperties = serde_json::from_str(response_as_str).unwrap();
            websocket_connection_properties
        })
        .map_err(|err| {
            println!("Error: {}", err);
        });

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let websocket_connection_properties: WebsocketConnectionProperties = rt.block_on(future).unwrap();

        println!("RESULT: {:?}", websocket_connection_properties);

        websocket_connection_properties
    }

    fn get_client() -> Client<HttpsConnector<HttpConnector>, Body> {
        let https = HttpsConnector::new(4).unwrap();
        Client::builder().build::<_, hyper::Body>(https)
    }

    fn build_request(&self) -> Request<Body> {
        let uri = "https://discordapp.com/api/v6/gateway/bot";
        Request::get(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, "Bot ".to_owned() + self.bot_token.as_str())
            .header(header::USER_AGENT, "TheBob/0.1")
            .body(Body::empty()).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct WebsocketConnectionProperties {
    url: String,
    shards: i32,
    session_start_limit: SessionStartLimit
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionStartLimit {
    total: i32,
    remaining: i32,
    reset_after: i32
}

impl WebsocketConnectionProperties {
    pub fn get_explicit_websocket_url(&self) -> String {
        self.url.clone() + "?v=6&encoding=json"
    }
}