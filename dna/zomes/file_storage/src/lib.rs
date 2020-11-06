extern crate file_storage:
use holochain_serialized_bytes::prelude::*;



//TODO: finish this function
// This callback event is calling after file_storage module uploaded a new file.
fn post_commit(headers){
  // Indexing file. 
  // 1) add path to file from Year,
  // 2) add path to file from Month,
  // 3) add path to file from Day
  // 4) add path to file from Hour
  // 5) add path to file from Minute

  // or add a path to file   Year.Month.Day.Hour.Minute
}