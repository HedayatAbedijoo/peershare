use hdk3::prelude::*;

extern crate file_storage;

// TODO1: link from my_address to new uploaded file should be happened here. "all_my_files"
// post_commit

/// TODO2: since the link of "all_my_files" created in this zome, get_all_my_files() zome function shoule be implemented here

///Example:  Adding extra functionality to the external module
#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct FilesResult {
    pub list: usize,
}

#[hdk_extern]
fn new_extention_function(_: ()) -> ExternResult<FilesResult> {
    Ok(FilesResult { list: 123 })
}
