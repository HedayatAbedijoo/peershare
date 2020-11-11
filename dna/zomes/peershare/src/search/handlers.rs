const MAX_RESULT_IN_PAGE: i32 = 50; /// approximate records in each page. We should query DHT in a loop whether we reach the filter_boundry or mx number in each page(scroll)
const STEP_IN_MIN: i32 = 1; // each page or step of search is X minute.

pub fn search_file_by_tags(tags:Vec<String>,current:u64, end_of_filter:u64) -> Result<(Vec<FileInfo>,u64)>{

    let mut start_pointer = current.clone();    

    // Start While:  result.lenght<= MAX_RESULT_IN_PAGE || start_pointer<=end_of_filter

    let result:Vec<String>;
    let path_date = "20201206".into(); // get YearMonthDay in this format YYYYMMDD based on the start_pointer
    let path_time = "1312".into(); // get time in this format HHMM based on the start_pointer

    for s in tags{
    // Gnerate TAGS: (format!("{}.{}.{}",s,path_date,path_time));
    // Query each Tag from DHT and push to result
    }
    start_pointer+=STEP_IN_MIN;

    //  END While
    Ok((result,start_pointer))

}