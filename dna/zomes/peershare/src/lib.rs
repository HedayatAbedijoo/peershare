use hdk3::prelude::*;

mod calendar_event;
mod utils;



pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}


//TODO: finish this function 
#[derive(Serialize, Deserialize, SerializedBytes)]
struct SearchInput{
    tags:Vec<String>,
    filter_by_last_min: u32 // filter by last X minutes.  UI  show Year,Month,Day,Hour, Min and calculate the minutes before calling zome
}



#[derive(Serialize, Deserialize, SerializedBytes)]
struct FileInfo{
    hash:EntryHash,
    file_name:String,
    file_size:u32,
    owner:String
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct SearchResult{
    result:Vec<FileInfo>,
    msg:String,
    status:bool
}


//TODO: Finish this function
#[hdk_extern]
pub fn create_tag(input:SearchInput)->SearchResult{
    // calculate  FilterTime =  Year.Month.Day.Hour.Min from SearchInput.filter_by_last_min based on (now)
    // then we would have 2 sources of date: 
    // from  FilterTime to now based on each minutes(or quarter)
    // get path Year.Month.Day.Hour.Min
    // Collect all data and result
    Ok( SearchResult{status:true, msg:"".into()})
}
