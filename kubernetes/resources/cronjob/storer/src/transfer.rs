use googapis::{
    google::storage::v1::{storage_client::StorageClient, InsertObjectRequest},
    CERTIFICATES,
};
use gouth::Token;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use tonic::IntoStreamingRequest;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

use tonic::codegen::futures_core::Stream;

use std::env;

async fn download() -> Result<(), Box<dyn std::error::Error>> {
    let target = "https://github.com/twbs/bootstrap/archive/v4.0.0.zip";
    let response = reqwest::get(target).await?;

    let path = Path::new("./download.zip");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
struct CustomRequest {}
struct CustomStream {}

impl Stream for CustomStream {
    type Item = InsertObjectRequest;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<InsertObjectRequest>> {
        todo!()
    }
}

impl IntoStreamingRequest for CustomRequest {
    type Stream = CustomStream;

    type Message = InsertObjectRequest;

    fn into_streaming_request(self) -> Request<CustomStream> {
        todo!()
    }
}

pub async fn upload() -> Result<(), Box<dyn std::error::Error>> {
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name("storage.googleapis.com");

    let channel = Channel::from_static("https://storage.googleapis.com")
        .tls_config(tls_config)?
        .connect()
        .await?;

    let token = Token::new()?;
    let mut service = StorageClient::with_interceptor(channel, move |mut req: Request<()>| {
        let token = &*token.header_value().unwrap();
        let meta = MetadataValue::from_str(token).unwrap();
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    });

    let project = env::var("GCP_PROJECT").expect("GCP_PROJECT must be set");
    let request = CustomRequest {};
    let response = service.insert_object(request).await;
    println!("RESPONSE={:?}", response);
    Ok(())
}

#[tokio::main]
pub async fn transfer() -> Result<(), Box<dyn std::error::Error>> {
    // download();
    upload();
    Ok(())
}
