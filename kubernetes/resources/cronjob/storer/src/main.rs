extern crate storer;

use dotenv::dotenv;
use storer::transfer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let input_data = env::var("INPUT_DATA").expect("INPUT_DATA must be set");
    // let parsed_data = parse(input_data);
    transfer().await;
}
