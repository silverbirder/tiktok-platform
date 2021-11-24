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
        let mut stats_fields: HashMap<String, Value> = HashMap::new();
        let mut author_fields: HashMap<String, Value> = HashMap::new();
        let mut music_fields: HashMap<String, Value> = HashMap::new();
        let mut author_stats_fields: HashMap<String, Value> = HashMap::new();
        fields.insert(
            String::from("id"),
            Value {
                value_type: Some(ValueType::StringValue(self.id.clone())),
            },
        );
        fields.insert(
            String::from("create_time"),
            Value {
                value_type: Some(ValueType::IntegerValue(self.create_time.clone())),
            },
        );
        video_fields.insert(
            String::from("download_addr"),
            Value {
                value_type: Some(ValueType::StringValue(self.video.download_addr.clone())),
            },
        );
        video_fields.insert(
            String::from("dynamic_cover"),
            Value {
                value_type: Some(ValueType::StringValue(self.video.dynamic_cover.clone())),
            },
        );
        video_fields.insert(
            String::from("cover"),
            Value {
                value_type: Some(ValueType::StringValue(self.video.cover.clone())),
            },
        );
        video_fields.insert(
            String::from("format"),
            Value {
                value_type: Some(ValueType::StringValue(self.video.format.clone())),
            },
        );
        video_fields.insert(
            String::from("duration"),
            Value {
                value_type: Some(ValueType::IntegerValue(self.video.duration.clone())),
            },
        );
        fields.insert(
            String::from("video"),
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: video_fields,
                })),
            },
        );
        stats_fields.insert(
            String::from("digg_count"),
            Value {
                value_type: Some(ValueType::IntegerValue(self.stats.digg_count.clone())),
            },
        );
        stats_fields.insert(
            String::from("share_count"),
            Value {
                value_type: Some(ValueType::IntegerValue(self.stats.share_count.clone())),
            },
        );
        stats_fields.insert(
            String::from("comment_count"),
            Value {
                value_type: Some(ValueType::IntegerValue(self.stats.comment_count.clone())),
            },
        );
        stats_fields.insert(
            String::from("play_count"),
            Value {
                value_type: Some(ValueType::IntegerValue(self.stats.play_count.clone())),
            },
        );
        fields.insert(
            String::from("stats"),
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: stats_fields,
                })),
            },
        );
        author_fields.insert(
            String::from("id"),
            Value {
                value_type: Some(ValueType::StringValue(self.author.id.clone())),
            },
        );
        author_fields.insert(
            String::from("unique_id"),
            Value {
                value_type: Some(ValueType::StringValue(self.author.unique_id.clone())),
            },
        );
        author_fields.insert(
            String::from("avatar_medium"),
            Value {
                value_type: Some(ValueType::StringValue(self.author.avatar_medium.clone())),
            },
        );
        fields.insert(
            String::from("author"),
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: author_fields,
                })),
            },
        );
        music_fields.insert(
            String::from("id"),
            Value {
                value_type: Some(ValueType::StringValue(self.music.id.clone())),
            },
        );
        music_fields.insert(
            String::from("title"),
            Value {
                value_type: Some(ValueType::StringValue(self.music.title.clone())),
            },
        );
        music_fields.insert(
            String::from("cover_medium"),
            Value {
                value_type: Some(ValueType::StringValue(self.music.cover_medium.clone())),
            },
        );
        music_fields.insert(
            String::from("author_name"),
            Value {
                value_type: Some(ValueType::StringValue(self.music.author_name.clone())),
            },
        );
        music_fields.insert(
            String::from("original"),
            Value {
                value_type: Some(ValueType::BooleanValue(self.music.original.clone())),
            },
        );
        music_fields.insert(
            String::from("duration"),
            Value {
                value_type: Some(ValueType::IntegerValue(self.music.duration.clone())),
            },
        );
        music_fields.insert(
            String::from("album"),
            Value {
                value_type: Some(ValueType::StringValue(self.music.album.clone())),
            },
        );
        fields.insert(
            String::from("music"),
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: music_fields,
                })),
            },
        );
        author_stats_fields.insert(
            String::from("following_count"),
            Value {
                value_type: Some(ValueType::IntegerValue(
                    self.author_stats.following_count.clone(),
                )),
            },
        );
        author_stats_fields.insert(
            String::from("follower_count"),
            Value {
                value_type: Some(ValueType::IntegerValue(
                    self.author_stats.follower_count.clone(),
                )),
            },
        );
        author_stats_fields.insert(
            String::from("heart_count"),
            Value {
                value_type: Some(ValueType::IntegerValue(
                    self.author_stats.heart_count.clone(),
                )),
            },
        );
        author_stats_fields.insert(
            String::from("video_count"),
            Value {
                value_type: Some(ValueType::IntegerValue(
                    self.author_stats.video_count.clone(),
                )),
            },
        );
        author_stats_fields.insert(
            String::from("digg_count"),
            Value {
                value_type: Some(ValueType::IntegerValue(
                    self.author_stats.digg_count.clone(),
                )),
            },
        );
        author_stats_fields.insert(
            String::from("heart"),
            Value {
                value_type: Some(ValueType::IntegerValue(self.author_stats.heart.clone())),
            },
        );
        fields.insert(
            String::from("author_stats"),
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: author_stats_fields,
                })),
            },
        );
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
