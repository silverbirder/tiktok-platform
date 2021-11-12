use googapis::{
    google::storagetransfer::v1::{
        storage_transfer_service_client::StorageTransferServiceClient, CreateTransferJobRequest,
    },
    CERTIFICATES,
};
use gouth::Token;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

use crate::parser::Input;

use std::env;

#[tokio::main]
pub async fn transfer(data: Input) -> Result<(), Box<dyn std::error::Error>> {
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name("storagetransfer.googleapis.com");

    let channel = Channel::from_static("https://storagetransfer.googleapis.com")
        .tls_config(tls_config)?
        .connect()
        .await?;

    let token = Token::new()?;
    let mut service =
        StorageTransferServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            let token = &*token.header_value().unwrap();
            let meta = MetadataValue::from_str(token).unwrap();
            req.metadata_mut().insert("authorization", meta);
            Ok(req)
        });

    let project = env::var("GCP_PROJECT").expect("GCP_PROJECT must be set");
    service
        .create_transfer_job(CreateTransferJobRequest {
            ..Default::default()
        })
        .await?;
    Ok(())
}
