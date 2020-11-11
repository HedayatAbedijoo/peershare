use hdk3::prelude::*;

mod tag;

//TODO: Finish this function
#[hdk_extern]
pub fn create_tag(input:CreateTag)->CreateTagResult<T>{   
    
     tag::handlers::create_tag(input);

    Ok( CreateTag{result:true, msg:"".into()})
}





