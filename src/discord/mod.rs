mod gateway;
mod properties;

use std::str;

use hyper::{Client, Method, Request, Body};
use hyper::header::HeaderValue;
use hyper::rt::{self, Future, Stream};

use self::gateway::opcode::{self, OpcodeValue};

pub fn authenticate_bot(token: String) {

    let opcode = OpcodeValue::Identify;

    println!("opcode: {:?}", opcode::get_opcode_value(opcode));
}

//TODO postman call hier rein
pub fn get_gateway_information() {

    let discord_header_location = "src/resources/discord_header";
    let discord_api_location = "src/resources/discord_api";
    let discord_properties = properties::read_discord_properties(discord_header_location,discord_api_location);

    println!("discord properties: {:?}", discord_properties);

//    let client = Client::new();
//
//    let json = r#"{"library":"hyper"}"#;
//    let uri: hyper::Uri = "http://httpbin.org/post".parse().unwrap();
//    let mut req = Request::new(Body::from(json));
//    *req.method_mut() = Method::POST;
//    *req.uri_mut() = uri.clone();
//    req.headers_mut().insert(
//        hyper::header::CONTENT_TYPE,HeaderValue::from_static("application/json")
//    );
//
//    let post = client.request(req).and_then(|res| {
//        println!("POST RESPONSE STATUS: {}", res.status());
//
//        res.into_body().concat2()
//    });
//
//    post
//        .map(|(posted)| {
//            println!("POST RESPONSE BODY: {:?}", str::from_utf8(&posted).unwrap());
//        })
//        .map_err(|err| {
//            println!("Error: {}", err);
//        })
}