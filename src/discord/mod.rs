use std::str;

use tokio;

use hyper::{header, Client, Request, Body};
use hyper::client::HttpConnector;
use hyper::rt::{self, Future, Stream};

use hyper_tls::HttpsConnector;

use serde_json::{self, Value, Error};

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
        Self::ask_for_websocket_information(self);
    }

    fn ask_for_websocket_information(&self) {
        let client = Self::get_client();
        let req = Self::build_request(self);

        let response_stream = client.request(req).and_then(|res| {
            println!("POST RESPONSE STATUS: {}", res.status());

            res.into_body().concat2()
        });

        let future = response_stream.map(|get_chunk| -> String {
            let response_as_str = str::from_utf8(&get_chunk).unwrap();
            println!("POST RESPONSE BODY: {:?}", response_as_str);

//            let gateway_information: WebsocketConnectionProperties = serde_json::from_str(response_as_str).unwrap();
//            println!("GATEWAY RESPONSE: {}", gateway_information.url);
//            println!("GATEWAY RESPONSE: {}", gateway_information.session_start_limit.reset_after);

            response_as_str.to_string()
        })
        .map_err(|err| {
            println!("Error: {}", err);
        });


        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let response_body = rt.block_on(future).unwrap();

        println!("RESULT: {:?}", response_body);
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