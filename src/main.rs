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
    response.read_to_string(&mut buffer).expect("Bing response was empty");

    let response_json = json::Json::from_str(&buffer).unwrap();
    let json_obj = response_json.as_object().unwrap();
    let images = json_obj.get("images").unwrap();
    let json_array = images.as_array().unwrap();
    let image = json_array[0].as_object().unwrap();
    let img_url = image.get("url").unwrap();
    let img_request_url = "https://www.bing.com".to_string() + &img_url.as_string().unwrap();

    let mut img_data_response = http_client.get(&img_request_url).send().unwrap();
    assert_eq!(img_data_response.status, hyper::Ok);

    let mut img_buf = Vec::new();
    img_data_response.read_to_end(&mut img_buf).expect("Bing image response was empty");
}