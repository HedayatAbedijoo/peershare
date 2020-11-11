#[derive(Serialize, Deserialize, SerializedBytes)]
struct CreateTag {
    file_hash: EntryHash,
    tags: Vec<String>,
}


#[derive(Serialize, Deserialize, SerializedBytes)]
struct CreateTagResult{
    result:bool,
    msg:String
}