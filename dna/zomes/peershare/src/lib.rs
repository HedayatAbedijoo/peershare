use hdk3::prelude::*;

mod calendar_event;
mod utils;

// TODO: Actually code the zome, all this code is just for reference and quick copy-paste

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}


//todo 1: 
// 



