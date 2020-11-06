use hdk3::prelude::*;
mod utils;

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

//TODO: Finish this function
#[hdk_extern]
pub fn create_tag(input:CreateTag)->CreateTagResult{
    // 1- check for double entry and select unique tags
    // 2- call evaluation function to validate all tags
    // 3- create path to hash_file for each tag
    Ok( CreateTag{result:true, msg:"".into()})
}





