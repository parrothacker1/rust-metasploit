#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
use std::collections::HashMap;
use error::MsfError;
use structs::modules;
pub struct compactible {
    pub name:String,
    pub client:Client,
}
pub struct list {
    pub client:Client,
}
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
impl list{
    pub fn new(client:Client) -> Self {
        list {
            client:client,
        }
    }
    pub fn exploits() -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
    pub fn auxiliary() -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
    pub fn post() -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
    pub fn payloads() -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
    pub fn encoders() -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
    pub fn nops() -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
}
pub fn info(client:Client,moduletype:String,modulename:String) -> Result<modules::info,MsfError> {
    let test:modules::info;
    Ok(test)
}
impl compactible {
    pub fn new(modulename:String,client:Client) -> Self {
        compactible {
            name:modulename,
            client:client,
        }
    }
    pub fn payload() -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
    pub fn target_payload(targetindx:i32) -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
    pub fn sessions() -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }

}
pub fn encoder(client:Client,data:String,encodermodule:String,options:HashMap<String,String>) -> Result<String,MsfError> {
    let test:String=String::new();
    Ok(test)
}
pub fn execute(client:Client,moduletype:String,modulename:String) -> Result<HashMap<String,String>,MsfError> {
    let test:HashMap<String,String>=HashMap::new();
    Ok(test)
}
