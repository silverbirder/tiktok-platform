/* 
The tiktok video download doesn't seem to be available by simple request...
*/

// use cloud_storage::Client;
// use url::Url;
// use std::env;
// use std::io::Cursor;
// use std::path::Path;
// use std::{fs::File, io::Read};


// pub async fn transfer(url: &str) -> String {
//     let response = reqwest::get(url).await.unwrap();
//     let paresed_url = Url::parse(url).unwrap();
//     let segments = paresed_url.path_segments().unwrap();
//     // let last_segment = segments.last().unwrap();
//     let last_segment = "a.mp4";
//     let path = Path::new(last_segment);
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}", why),
//         Ok(file) => file,
//     };
//     let mut content = Cursor::new(response.bytes().await.unwrap());
//     std::io::copy(&mut content, &mut file).unwrap();

//     let project = env::var("GCP_PROJECT").expect("GCP_PROJECT must be set");
//     let client = Client::default();
//     let bucket_name = String::from(project.to_string() + &"-bucket".to_string());
//     let mut bytes: Vec<u8> = Vec::new();
//     for byte in File::open(last_segment).unwrap().bytes() {
//         bytes.push(byte.unwrap())
//     }
//     let res = client
//         .object()
//         .create(&bucket_name, bytes, last_segment, "text/plain")
//         .await
//         .unwrap();
//     let storage_url = [
//         "https://storage.googleapis.com/",
//         bucket_name.as_str(),
//         "/",
//         last_segment,
//     ]
//     .concat();
//     return storage_url;
// }
