use cloud_storage::{Client, NewBucket};
use std::env;
use std::io::Cursor;
use std::path::Path;
use std::{fs::File, io::Read};
use url::{Url};

pub async fn transfer(url: &str) -> () {
    let response = reqwest::get(url).await.unwrap();
    let paresed_url = Url::parse(url).unwrap();
    let segments = paresed_url.path_segments().unwrap();
    let last_segment = segments.last().unwrap();
    let path = Path::new(last_segment);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let mut content = Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut file).unwrap();

    let project = env::var("GCP_PROJECT").expect("GCP_PROJECT must be set");
    let client = Client::default();
    let bucket_name = String::from(project + &"-bucket".to_string());
    // match client
    //     .bucket()
    //     .create(&NewBucket {
    //         name: bucket_name.to_string(),
    //         ..Default::default()
    //     })
    //     .await
    // {
    //     Err(err) => {
    //         println!("{:?}", err);
    //     }
    //     Ok(res) => {
    //         println!("{:?}", res);
    //     }
    // };
    let mut bytes: Vec<u8> = Vec::new();
    for byte in File::open(last_segment).unwrap().bytes() {
        bytes.push(byte.unwrap())
    }
    match client
        .object()
        .create(
            &bucket_name,
            bytes,
            last_segment,
            "text/plain",
        )
        .await
    {
        Err(err) => {
            println!("{:?}", err);
        }
        Ok(res) => {
            println!("{:?}", res);
            // let a = "https://storage.cloud.google.com/".to_string() + project.clone().to_string().as_str() + "/" + last_segment;
            // let a = "https://storage.cloud.google.com/".to_string() + project;
            // ["https://storage.cloud.google.com/", project].connect("");
        }
    };
}
