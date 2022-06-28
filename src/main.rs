extern crate rustc_serialize;

use rustc_serialize::json;
use std::error::Error;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let markets = [
        "ja-JP", "en-US", "en-UK", "en-AU", "en-NZ", "en-CA", "de-DE", "zh-CN",
    ];
    let request_url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt="
        .to_string()
        + &markets[0];

    let buffer = reqwest::blocking::get(&request_url)?.text()?;
    let response_json = json::Json::from_str(&buffer)?;
    let json_obj = response_json.as_object().unwrap();
    let images = json_obj.get("images").unwrap();
    let json_array = images.as_array().unwrap();
    let image = json_array[0].as_object().unwrap();
    let img_url = image.get("url").unwrap();

    let img_request_url = "https://www.bing.com".to_string() + &img_url.as_string().unwrap();
    let img_data_response = reqwest::blocking::get(&img_request_url)?.bytes()?;

    let save_path = "/usr/share/antergos/wallpapers";
    let result_filename = save_path.to_string() + &"/bing_wallpaper.jpg";
    let mut output = std::fs::File::create(&result_filename)?;
    output
        .write(&img_data_response)
        .expect("Overwriting the latest wallpaper didn't work");

    Ok(())
}
