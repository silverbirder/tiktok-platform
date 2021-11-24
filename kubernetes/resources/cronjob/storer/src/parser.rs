use serde::{Deserialize, Serialize};
use serde_json;

use googapis::google::firestore::v1::{value::ValueType, Document, MapValue, Value};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub id: String,
    pub create_time: i64,
    pub video: Video,
    pub stats: Stats,
    pub author: Author,
    pub music: Music,
    pub author_stats: AuthorStats,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub download_addr: String,
    pub dynamic_cover: String,
    pub cover: String,
    pub format: String,
    pub duration: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub digg_count: i64,
    pub share_count: i64,
    pub comment_count: i64,
    pub play_count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub unique_id: String,
    pub avatar_medium: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthorStats {
    pub following_count: i64,
    pub follower_count: i64,
    pub heart_count: i64,
    pub video_count: i64,
    pub digg_count: i64,
    pub heart: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Music {
    pub id: String,
    pub title: String,
    pub cover_medium: String,
    pub author_name: String,
    pub original: bool,
    pub duration: i64,
    pub album: String,
}

impl Input {
    pub fn to_document(&self, name: &str) -> Option<Document> {
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

pub fn parse(str: String) -> Input {
    let v: Input = serde_json::from_str(&str).unwrap();
    return v;
}
