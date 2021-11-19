use cloud_storage::{Client, NewBucket};
use std::env;
use std::io::Cursor;
use std::path::Path;
use std::{fs::File, io::Read};

pub async fn transfer() -> () {
    let target = "https://github.com/twbs/bootstrap/archive/v4.0.0.zip";
    let response = reqwest::get(target).await.unwrap();

    let path = Path::new("./download.zip");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let mut content = Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut file).unwrap();

    let project = env::var("GCP_PROJECT").expect("GCP_PROJECT must be set");
    let client = Client::default();
    let bucket_name = project + &"-bucket".to_string();
    match client
        .bucket()
        .create(&NewBucket {
            name: bucket_name,
            ..Default::default()
        })
        .await
    {
        Err(err) => {
            println!("{:?}", err);
        }
        Ok(res) => {
            println!("{:?}", res);
        }
    };
    let mut bytes: Vec<u8> = Vec::new();
    for byte in File::open("./README.md").unwrap().bytes() {
        bytes.push(byte.unwrap())
    }
    match client
        .object()
        .create(
            "xxx-bucket",
            bytes,
            "README.md",
            "text/plain",
        )
        .await
    {
        Err(err) => {
            println!("{:?}", err);
        }
        Ok(res) => {
            println!("{:?}", res);
        }
    };
}
