use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub id: String,
    pub createTime: i64,
}

pub fn parse(str: String) -> Input {
    let v: Input = serde_json::from_str(&str).unwrap();
    return v;
}
