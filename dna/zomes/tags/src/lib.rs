//use hdk3::prelude::*;
//use hc_utils::WrappedEntryHash;
use crate::tag::entry::{CreateTagInput, TagResult};
use hdk3::prelude::*;
mod tag;

#[hdk_extern]
pub fn get_agent_pubkey(_: ()) -> ExternResult<AgentPubKey> {
    let agent_info = agent_info()?;
    Ok(agent_info.agent_latest_pubkey)
}

#[hdk_extern]
pub fn tag_file(input: CreateTagInput) -> ExternResult<TagResult> {
    return Ok(tag::handlers::create_tags(input));
}
