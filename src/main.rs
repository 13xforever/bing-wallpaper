extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Chunk, Client};
use tokio_core::reactor::Core;
use serde_json::Value;

fn main() {
    let markets = [
        "ja-JP", "en-US", "en-UK", "en-AU", "en-NZ", "en-CA", "de-DE", "zh-CN"
    ];
    let request_url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt="
        .to_string() + &markets[0];
    let request_uri = request_url.parse().unwrap();

    let mut core = Core::new().unwrap();
    let core_handle = core.handle();
    let http_client = Client::new(&core_handle);
    let request = http_client.get(request_uri).and_then(|res| {
        assert_eq!(res.status(), hyper::Ok);

        res.body().concat2().and_then(move |body: Chunk| {
            let response_json: Value = serde_json::from_slice(&body).unwrap();
            let img_url = response_json["images"][0]["url"];
            let img_request_url = "https://www.bing.com".to_string() + &img_url.as_str().unwrap();
            let img_request_uri = img_request_url.parse().unwrap();

            let mut img_data_response = http_client.get(img_request_uri).and_then(|data_res| {
                assert_eq!(img_data_response.status, hyper::Ok);

                //let mut img_buf = Vec::new();
                //img_data_response.read_to_end(&mut img_buf).expect("Bing image response was empty");

                //compare md5

                //let mut result_path = std::env::home_dir().unwrap();
                //result_path.push("Pictures");
                //result_path.push("Bing");
                let save_path = "/usr/share/antergos/wallpapers";
                //println!("Trying to create directory {:?}", save_path);
                //std::fs::create_dir_all(save_path).expect("Couldn't create output directory");

                let result_filename = save_path.to_string() + &"/bing_wallpaper.jpg";
                //println!("Trying to save as {:?}", result_filename);
                let mut output = std::fs::File::create(&result_filename).unwrap();
                //output.write_all(&img_buf);
                std::io::copy(&mut img_data_response, &mut output)
                    .expect("Overwritting the latest wallpaper didn't work");
            });

            Ok(())
        })
    });
    core.run(request).unwrap();
}
