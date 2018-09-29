use std::str;

use hyper::{Client, Method, Request, Body};
use hyper::header::HeaderValue;
use hyper::rt::{self, Future, Stream};

const DISCORD_API_VERSION: &str = "https://discordapp.com/api/v6";

pub fn authenticate_bot() -> impl Future<Item=(), Error=()> {

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

pub fn run_post() {
    rt::run(authenticate_bot());
}

pub fn run_get() {
    rt::run(send_get());
}