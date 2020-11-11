use hdk3::prelude::*;

mod calendar_event;
mod search;



pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}


//TODO: Finish this function
#[hdk_extern]
pub fn browse_files(input:SearchInput)->SearchResult{
  
    let search_result = search::handlers::search_file_by_tags(input.tags, input.from_timestamp, input.filter_boundry_in_min);
    
    Ok( SearchResult{
        status:true, 
        msg:"".into(), 
        result: search_results.0, // search result 
        from_timestamp: search_result.1, // the starting point of search, shifted back to this point, for the next round of paging.
        filter_boundry_in_min:input.filter_boundry_in_min // the end point of filtering, by passing via this variable.
    })
}
