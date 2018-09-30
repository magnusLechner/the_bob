use std::str;

use hyper::{Client, Method, Request, Body};
use hyper::header::HeaderValue;
use hyper::rt::{self, Future, Stream};

const DISCORD_BASE_UTL: &str = "https://discordapp.com/api/";
const DISCORD_API_VERSION: &str = "v6/";

//TODO Change String to str or &str
//TODO Reorganize structs

struct Authentication {
    header: String,
    token_type: String,
    token: String
}

fn build_authentication(token: String) -> Authentication {
    Authentication {
        header: String::from("Authorization"),
        token_type: String::from("Bot"),
        token
    }
}

struct DiscordApiSettings {
    header: String,
    user_agent: String,
    rate_limit: u8
}

//TODO Every route has a different rate_limit which must be obeyed
// https://discordapp.com/developers/docs/topics/rate-limits#rate-limits
fn build_discord_api_setting(rate_limit: u8) -> DiscordApiSettings {
    DiscordApiSettings {
        header: String::from("User-Agent"),
        user_agent: String::from("TheBob/0.1"),
        rate_limit
    }
}

pub fn authenticate_bot(token: String) -> impl Future<Item=(), Error=()> {

    let authentication = build_authentication(token);
    let discord_api_settings = build_discord_api_setting(5);

    let client = Client::new();

    // still inside rt::run...
    let json = r#"{"library":"hyper"}"#;
    let uri: hyper::Uri = "http://httpbin.org/post".parse().unwrap();
    let mut req = Request::new(Body::from(json));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,HeaderValue::from_static("application/json")
    );

    let post = client.request(req).and_then(|res| {
        println!("POST RESPONSE STATUS: {}", res.status());

        res.into_body().concat2()
    });

    post
        .map(|(posted)| {
            println!("POST RESPONSE BODY: {:?}", str::from_utf8(&posted).unwrap());
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
}

pub fn send_post() -> impl Future<Item=(), Error=()> {

    let client = Client::new();

    // still inside rt::run...
    let json = r#"{"library":"hyper"}"#;
    let uri: hyper::Uri = "http://httpbin.org/post".parse().unwrap();
    let mut req = Request::new(Body::from(json));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,HeaderValue::from_static("application/json")
    );

    let post = client.request(req).and_then(|res| {
        println!("POST RESPONSE STATUS: {}", res.status());

        res.into_body().concat2()
    });

    post
        .map(|(posted)| {
            println!("POST RESPONSE BODY: {:?}", str::from_utf8(&posted).unwrap());
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
}

fn send_get() -> impl Future<Item=(), Error=()>{

    let client = Client::new();

    let uri = "http://httpbin.org/ip".parse().unwrap();

    client
        .get(uri)
        .map(|res| {
            println!("GET RESPONSE STATUS: {}", res.status());
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
}

pub fn run_authentication(token: String) {
    rt::run(authenticate_bot(token));
}

pub fn run_post() {
    rt::run(send_post());
}

pub fn run_get() {
    rt::run(send_get());
}