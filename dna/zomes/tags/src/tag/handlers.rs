use crate::tag::validation::validate_tags;
use crate::CreateTagInput;
use crate::TagResult;

pub fn create_tags(input: CreateTagInput) -> TagResult {
    let validate: TagResult = validate_tags(input.tags.clone());
    if !validate.result {
        return validate;
    }
    return TagResult::success();
}

// pub fn create_tags()->Result{

//     let tag_list:Vec<String>  = tag_util::get_valid_tags(input.tags);
//     //if tag_list return error,
//     // return Ok( CreateTag{result:false, msg:function_error msg})

//     let path_date = "20201206".into(); // get current YearMonthDay in this format YYYYMMDD
//     let path_time = "1312".into(); // get current time in this format HHMM
//     // tags:   art;old

//     // path1:   art.20201206.1312
//     // path1:   old.20201206.1312

//     for s in tag_list { // each String is moved into s here...
//         // exp:   tag.YearMonthDay.HHmm
//         //        art.20200623.1607
//         let path = format!("{}.{}.{}",s,path_date,path_time);
//         // create path to EntryHash
//     }

// }
