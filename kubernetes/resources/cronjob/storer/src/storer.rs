use googapis::{
    google::firestore::v1::{
        firestore_client::FirestoreClient, value::ValueType, Document, MapValue,
        UpdateDocumentRequest, Value,
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
    fn to_document(&self, name: &str) -> Option<Document> {
        let mut fields: HashMap<String, Value> = HashMap::new();
        let mut video_fields: HashMap<String, Value> = HashMap::new();
        // let mut user_fields: HashMap<String, Value> = HashMap::new();
        fields.insert(
            String::from("id"),
            Value {
                value_type: Some(ValueType::StringValue(self.id.clone())),
            },
        );
        video_fields.insert(
            String::from("download_addr"),
            Value {
                value_type: Some(ValueType::StringValue(self.video.download_addr.clone())),
            },
        );
        // video_fields.insert(
        //     String::from("original_url"),
        //     Value {
        //         value_type: Some(ValueType::StringValue(self.video.original_url.clone())),
        //     },
        // );
        fields.insert(
            String::from("video"),
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: video_fields,
                })),
            },
        );
        // user_fields.insert(
        //     String::from("name"),
        //     Value {
        //         value_type: Some(ValueType::StringValue(self.user.name.clone())),
        //     },
        // );
        // fields.insert(
        //     String::from("user"),
        //     Value {
        //         value_type: Some(ValueType::MapValue(MapValue {
        //             fields: user_fields,
        //         })),
        //     },
        // );
        Some(Document {
            fields: fields,
            name: name.to_string(),
            ..Default::default()
        })
    }
}

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
