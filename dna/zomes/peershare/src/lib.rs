use hdk3::prelude::*;
mod search;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

//TODO: Finish this function
// #[hdk_extern]
// pub fn browse_files(input:SearchInput)->SearchResult{
//     let search_result = search::handlers::search_file_by_tags(input.tags, input.from_timestamp, input.filter_boundry_in_min);
//     Ok( SearchResult{
//         status:true,
//         msg:"".into(),
//         result: search_results.0, // search result
//         from_timestamp: search_result.1, // the starting point of search, shifted back to this point, for the next round of paging.
//         filter_boundry_in_min:input.filter_boundry_in_min // the end point of filtering, by passing via this variable.
//     })
// }

// let links = get_links(env.clone(), base_address, zome_name, link_tag).await;
// let links = links
//     .into_inner()
//     .into_iter()
//     .map(|h| h.target.try_into().unwrap())
//     .collect::<Vec<EntryHash>>();

// #[hdk_extern]
// pub fn get_my_files(_:())
//pub fn someting() -> FileMetaData {}

// #[derive(Serialize, Deserialize, SerializedBytes)]
// pub struct FilesResult {
//     pub list: usize,
// }
// #[hdk_extern]
// fn get_my_files2(_: ()) -> ExternResult<FilesResult> {
//     let _linktag = link_tag("all_my_files").unwrap();
//     let links = get_links(my_address(), None)?.into_inner().len();
//     Ok(FilesResult { list: links })
// }

// #[derive(Serialize, Deserialize, SerializedBytes)]
// struct StringLinkTag(String);
// pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
//     let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
//     Ok(LinkTag(sb.bytes().clone()))
// }

// fn my_address() -> EntryHash {
//     let agent_info = agent_info().unwrap();
//     let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.into();
//     agent_address.into()
// }
