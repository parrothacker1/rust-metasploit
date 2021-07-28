#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
use common::{MsfError,Return_Type};
use error::conerr;
use structs::core;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn add_module_path(client:Client,path:String) -> Return_Type {
    let test:core::addmodpath;
    let con=connect::connect(client.url);
    match con {
		Ok(val) => {
			
			
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
    test
}
pub fn module_stats(client:Client) -> Result<core::modulestat,MsfError> {
    let test:core::modulestat;
    Ok(test)
}
pub fn reload_module(client:Client) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub fn save(client:Client) -> Result<bool,MsfError> {
    let test:bool = true;
    Ok(test)
}
pub fn setg(client:Client,name:String,value:String) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub fn unsetg(client:Client,name:String) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub fn thread_kill(client:Client,threadID:i32) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub fn version(client:Client) -> Result<core::version,MsfError> {
    let test:core::version;
    Ok(test)
}
pub fn stop(client:Client) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
