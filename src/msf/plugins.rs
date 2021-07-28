#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
use error::MsfError;
use structs::plugins;
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
pub fn load(client:Client,pluginname:String) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub fn unload(client:Client,pluginname:String) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub fn plugins(client:Client) -> Result<Vec<String>,MsfError> {
    let test:Vec<String>=Vec::new();
    Ok(test)
}

