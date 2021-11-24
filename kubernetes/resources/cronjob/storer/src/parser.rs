use serde::{Deserialize, Serialize};
use serde_json;

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

pub fn parse(str: String) -> Input {
    let v: Input = serde_json::from_str(&str).unwrap();
    return v;
}
