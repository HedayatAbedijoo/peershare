use crate::tag::validation::*;
use crate::CreateTagInput;
use crate::TagResult;
use hdk3::prelude::*;
// use std::time::Duration;
// use timestamp::Timestamp;
pub static ALL_MY_FILES: &str = "all_my_files";

pub fn create_tags(input: CreateTagInput) -> TagResult {
    if !is_hash_file_exist(&input.file_hash.0) {
        return TagResult::error("Hash file is invalid".into());
    }
    let validate: TagResult = validate_tags(input.tags.clone());
    if !validate.result {
        return validate;
    }

    let mut paths: Vec<String> = Vec::new();
    let current_date = get_current_year_month();
    let current_time = get_current_hour_min();
    for elem in input.tags {
        paths.push(generate_path(
            elem.clone().to_lowercase(),
            current_date.clone(),
            current_time.clone(),
        ));
    }

    // This should be in post_commit
    create_link_from_my_address_to_file(input.file_hash.0.clone());

    for _path in paths {
        // generate path
    }

    return TagResult::success();
}

// This should be relocated to post_commit of file_storage module
fn create_link_from_my_address_to_file(file_hash: EntryHash) {
    let _something = create_link(
        my_address().clone(),
        file_hash,
        link_tag(&ALL_MY_FILES).unwrap(),
    );
}

fn generate_path(tag: String, current_date: String, current_time: String) -> String {
    return format!("{}.{}.{}", tag, current_date, current_time);
}

fn get_current_year_month() -> String {
    "202012".into()
}

fn get_current_hour_min() -> String {
    "1213".into()
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}
pub fn my_address() -> EntryHash {
    let agent_info = agent_info().unwrap();
    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.into();
    agent_address.into()
}
// pub fn create_tags()->Result{

//     let tag_list:Vec<String>  = tag_util::get_valid_tags(input.tags);
//     //if tag_list return error,
//     // return Ok( CreateTag{result:false, msg:function_error msg})

//     let path_date = "20201206".into(); // get current YearMonthDay in this format YYYYMMDD
//     let path_time = "1312".into(); // get current time in this format HHMM
//     // tags:   art;old

//     // path1:   art.20201206.1312
//     // path1:   old.20201206.1312

//     for s in tag_list { // each String is moved into s here...
//         // exp:   tag.YearMonthDay.HHmm
//         //        art.20200623.1607
//         let path = format!("{}.{}.{}",s,path_date,path_time);
//         // create path to EntryHash
//     }

// }
