use crate::tag::entry::{CreateTagInput, TagResult};
use hdk3::prelude::*;
mod tag;
//use hc_utils::WrappedEntryHash;
#[hdk_extern]
pub fn get_agent_pubkey(_: ()) -> ExternResult<AgentPubKey> {
    let agent_info = agent_info()?;
    Ok(agent_info.agent_latest_pubkey)
}

#[hdk_extern]
pub fn tag_file(input: CreateTagInput) -> ExternResult<TagResult> {
    return Ok(tag::handlers::create_tags(input));
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct MyFilesResult {
    pub list: usize,
}

#[hdk_extern]
fn get_my_files(_: ()) -> ExternResult<MyFilesResult> {
    let _linktag = tag::handlers::link_tag(tag::handlers::ALL_MY_FILES).unwrap();
    let links = get_links(tag::handlers::my_address(), None)?.into_inner();
    Ok(MyFilesResult { list: links.len() })
}
