use hdk3::prelude::*;
#[hdk_extern]
pub fn get_agent_pubkey(_: ()) -> ExternResult<AgentPubKey> {
    let agent_info = agent_info()?;
    Ok(agent_info.agent_latest_pubkey)
}


#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct MyFilesResult {
    pub list: usize,
}


/// This code is duplicate here and into peershare zome, to replicate the issue, it will be removed later. 
pub static ALL_MY_FILES: &str = "all_my_files";
#[hdk_extern]
fn get_my_files(_: ()) -> ExternResult<MyFilesResult> {
    let links = get_links(my_address(), None)?.into_inner();
    Ok(MyFilesResult { list: links.len() })
}

pub fn my_address() -> EntryHash {
    let agent_info = agent_info().unwrap();
    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.into();
    agent_address.into()
}