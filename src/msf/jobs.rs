#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
use error::MsfError;
use structs::jobs;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn info(client:Client,jobid:String) -> Result<jobs::info,MsfError> {
    let test:jobs::info;
    Ok(test)
}
pub fn stop(client:Client,jobid:String) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
