extern crate storer;

use dotenv::dotenv;
use std::env;
use storer::{parse, transfer};

fn main() {
    dotenv().ok();
    let input_data = env::var("INPUT_DATA").expect("INPUT_DATA must be set");
    let parsed_data = parse(input_data);
    transfer(parsed_data);
}
