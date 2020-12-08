//use hdk3::prelude::*;
//use hc_utils::WrappedEntryHash;
use hdk3::prelude::*;

#[hdk_extern]
pub fn get_agent_pubkey(_: ()) -> ExternResult<AgentPubKey> {
    let agent_info = agent_info()?;
    Ok(agent_info.agent_latest_pubkey)
}
