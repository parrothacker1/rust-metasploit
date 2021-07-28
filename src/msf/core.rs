#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use common::{MsfError,Return_Type};
use error::conerr;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn add_module_path(client:Client,path:String) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
		Ok(val) => {
            if val.get("exploits")==None {
                let ret=MsfError {
                    error:val.get("error").unwrap().as_bool().unwrap(),
                    error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
                    error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
                };
                test=Return_Type::MsfErr(ret);
            } else {
                let ret=modules {
                    exploits:val.get("exploits").unwrap().as_i64().unwrap(),
                    auxiliary:val.get("auxiliary").unwrap().as_i64().unwrap(),
                    post:val.get("post").unwrap().as_i64().unwrap(),
                    encoders:val.get("encoders").unwrap().as_i64().unwrap(),
                    nops:val.get("nops").unwrap().as_i64().unwrap(),
                    payloads:val.get("payloads").unwrap().as_i64().unwrap(),
                };
                test=Return_Type::CoreModules(ret);
            }
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
    }
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
