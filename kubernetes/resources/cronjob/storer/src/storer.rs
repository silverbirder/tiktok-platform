use googapis::{
    google::firestore::v1::{firestore_client::FirestoreClient, UpdateDocumentRequest},
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

pub async fn store(data: Input) -> Result<(), Box<dyn std::error::Error>> {
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name("firestore.googleapis.com");

    let channel = Channel::from_static("https://firestore.googleapis.com")
        .tls_config(tls_config)?
        .connect()
        .await?;

    let token = Token::new()?;
    let mut service = FirestoreClient::with_interceptor(channel, move |mut req: Request<()>| {
        let token = &*token.header_value().unwrap();
        let meta = MetadataValue::from_str(token).unwrap();
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    });
    let project = env::var("GCP_PROJECT").expect("GCP_PROJECT must be set");
    // https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/createDocument
    let name = [
        "projects/",
        project.as_str(),
        "/databases/(default)/documents/collection/",
        data.id.as_str(),
    ]
    .concat();
    let response = service
        .update_document(Request::new(UpdateDocumentRequest {
            document: data.to_document(name.as_str()),
            update_mask: None,
            mask: None,
            current_document: None,
        }))
        .await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
