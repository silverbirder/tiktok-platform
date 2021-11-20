extern crate storer;

use dotenv::dotenv;
use storer::transfer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let input_data = env::var("INPUT_DATA").expect("INPUT_DATA must be set");
    // let parsed_data = parse(input_data);
    transfer("https://res.cloudinary.com/silverbirder/image/upload/w_1000,ar_16:9,c_fill,g_auto,e_sharpen/v1613137981/silver-birder.github.io/assets/IMG_3040.jpg").await;
}
