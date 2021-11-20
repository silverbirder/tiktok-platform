extern crate storer;

use dotenv::dotenv;
use std::env;
use storer::{parse, store, transfer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let input_data = env::var("INPUT_DATA").expect("INPUT_DATA must be set");
    let mut parsed_data = parse(input_data);
    let storage_url = transfer(parsed_data.video.download_addr.as_str()).await;
    parsed_data.video.download_addr = storage_url;
    store(parsed_data).await.unwrap();
}
