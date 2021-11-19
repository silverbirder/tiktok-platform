// use googapis::{
//     google::storage::v1::{storage_client::StorageClient, InsertObjectRequest},
//     CERTIFICATES,
// };
// use gouth::Token;
// use std::fs::File;
// use std::io::Cursor;
// use std::path::Path;
// use tonic::{metadata::MetadataMap, Extensions, IntoStreamingRequest};
// use tonic::{
//     metadata::MetadataValue,
//     transport::{Certificate, Channel, ClientTlsConfig},
//     Request,
// };

// use tonic::codegen::futures_core::Stream;

// use std::env;

// async fn download() -> Result<(), Box<dyn std::error::Error>> {
//     let target = "https://github.com/twbs/bootstrap/archive/v4.0.0.zip";
//     let response = reqwest::get(target).await?;

//     let path = Path::new("./download.zip");

//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}", why),
//         Ok(file) => file,
//     };
//     let mut content = Cursor::new(response.bytes().await?);
//     std::io::copy(&mut content, &mut file)?;
//     Ok(())
// }
// struct CustomRequest {
//     metadata: MetadataMap,
//     message: InsertObjectRequest,
//     extensions: Extensions,
// }
// struct CustomStream {}

// impl Stream for CustomStream {
//     type Item = InsertObjectRequest;

//     fn poll_next(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<InsertObjectRequest>> {
//         return std::task::Poll::Pending;
//         // return std::task::Poll::Ready(Some(self::Item));
//         // return std::task::Poll::Ready(None);
//     }
// }

// impl IntoStreamingRequest for CustomRequest {
//     type Stream = CustomStream;

//     type Message = InsertObjectRequest;

//     fn into_streaming_request(self) -> Request<CustomStream> {
//         return Request::new(Self::Stream {});
//     }
// }

// pub async fn transfer() -> () {
//     let tls_config = ClientTlsConfig::new()
//         .ca_certificate(Certificate::from_pem(CERTIFICATES))
//         .domain_name("storage.googleapis.com");

//     let channel = Channel::from_static("https://storage.googleapis.com")
//         .tls_config(tls_config).unwrap()
//         .connect()
//         .await.unwrap();

//     let token = Token::new().unwrap();
//     let mut service = StorageClient::with_interceptor(channel, move |mut req: Request<()>| {
//         let token = &*token.header_value().unwrap();
//         let meta = MetadataValue::from_str(token).unwrap();
//         req.metadata_mut().insert("authorization", meta);
//         Ok(req)
//     });

//     let project = env::var("GCP_PROJECT").expect("GCP_PROJECT must be set");
//     let insert_object_request = InsertObjectRequest {
//         write_offset: 0,
//         object_checksums: None,
//         finish_write: true,
//         common_object_request_params: None,
//         common_request_params: None,
//         first_message: None,
//         data: None,
//     };
//     let request = Request::new(insert_object_request);
//     let custom_request = CustomRequest {
//         metadata: request.metadata().clone(),
//         message: insert_object_request.clone(),
//         extensions: *request.extensions().clone(),
//     };
//     let response = service.insert_object(custom_request).await;
//     println!("RESPONSE={:?}", response);
// }
