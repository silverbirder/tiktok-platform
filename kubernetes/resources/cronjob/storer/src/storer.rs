use googapis::{
    google::firestore::v1::{
        firestore_client::FirestoreClient, value::ValueType, CreateDocumentRequest, Document, Value,
    },
    CERTIFICATES,
};
use gouth::Token;
use std::collections::HashMap;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

use crate::parser::Input;

use std::env;

impl Input {
    fn to_document(&self) -> Option<Document> {
        let mut fields: HashMap<String, Value> = HashMap::new();
        fields.insert(
            String::from("id"),
            Value {
                value_type: Some(ValueType::StringValue(self.id.clone())),
            },
        );
        fields.insert(
            String::from("create_time"),
            Value {
                value_type: Some(ValueType::IntegerValue(self.createTime.clone())),
            },
        );
        Some(Document {
            fields: fields,
            ..Default::default()
        })
    }
}

#[tokio::main]
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
    let response = service
        .create_document(Request::new(CreateDocumentRequest {
            parent: String::from("projects/")
                + project.as_str()
                + &String::from("/databases/(default)/documents/root/document"),
            collection_id: String::from("collection"),
            document_id: data.id.clone(),
            document: data.to_document(),
            ..Default::default()
        }))
        .await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
