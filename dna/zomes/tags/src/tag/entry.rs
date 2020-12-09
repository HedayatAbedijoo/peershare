use hc_utils::WrappedEntryHash;
use hdk3::prelude::*;

#[derive(Serialize, Deserialize, SerializedBytes, Clone)]
pub struct CreateTagInput {
    pub file_hash: WrappedEntryHash,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone)]
pub struct TagResult {
    pub result: bool,
    pub msg: String,
}

impl TagResult {
    pub fn success() -> Self {
        TagResult {
            result: true,
            msg: "".to_string(),
        }
    }

    pub fn error(err: String) -> Self {
        TagResult {
            result: false,
            msg: err,
        }
    }
}
