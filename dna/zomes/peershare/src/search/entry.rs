use hdk3::prelude::*;
#[derive(Serialize, Deserialize, SerializedBytes)]
struct SearchInput {
    tags: Vec<String>,
    from_timestamp: i64,        // search from this time-stamp
    filter_boundry_in_min: i64, // filter by last X minutes.  UI  show Year,Month,Day,Hour, Min and calculate the minutes before calling zome
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct SearchResult {
    result: Vec<FileInfo>,
    from_timestamp: i64,
    filter_boundry_in_min: i64,
    msg: String,
    status: bool,
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct FileInfo {
    hash: EntryHash,
    file_name: String,
    file_size: u32,
    owner: String,
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct MyFilesResult {
    pub list: Vec<EntryHash>,
}
