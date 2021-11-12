use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub id: String,
    pub create_time: i64,
}

pub fn parse(str: String) -> Input {
    let v: Input = serde_json::from_str(&str).unwrap();
    return v;
}