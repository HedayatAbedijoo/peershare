use hdk3::prelude::EntryHash;

use crate::TagResult;

pub fn validate_tags(tags: Vec<String>) -> TagResult {
    let mut err: Vec<String> = Vec::new();
    for elem in tags {
        if !is_tag_valid(&elem) {
            err.push(format!("{} is not a valid tag", elem));
        }
    }

    if err.len() == 0 {
        return TagResult::success();
    } else {
        return TagResult::error(err.join(";"));
    }
}
pub fn is_tag_valid(tag: &String) -> bool {
    tag.chars().all(char::is_alphabetic) && tag.len() >= 3 && tag.len() <= 10
}

pub fn is_hash_file_exist(_file_hash: &EntryHash) -> bool {
    true
}
