extern crate rustc_serialize;
extern crate hyper;

use std::io::Read;
use rustc_serialize::json;
use hyper::client::Client;

fn main() {
    let markets = ["ja-JP", "en-US", "en-UK", "en-AU", "en-NZ", "en-CA", "de-DE", "zh-CN"];
    let request_url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=".to_string() + &markets[0];
    let http_client = Client::new();
    let mut response = http_client.get(&request_url).send().unwrap();
    assert_eq!(response.status, hyper::Ok);

    let mut buffer = String::new();
    response.read_to_string(&mut buffer);

    let response_json = json::Json::from_str(&buffer.to_string()).unwrap();
}