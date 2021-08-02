#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use connect::connect;
use common::{ReturnValue as Return_Type,MsfError};

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
#[derive(se)]
struct sessionstop(String,String,String);
pub fn stop(client:Client,sessionid:String) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub enum shell {
    read(),
    write(),
}
impl shell {
    pub fn read(client:Client,sessionid:String,readpointer:Option<i32>) -> Result<sessions::shell_read,MsfError> {
        let test:sessions::shell_read;
        Ok(test)
    }
    pub fn write(client:Client,sessionid:String) -> Result<i32,MsfError> {
        let test:i32=1;
        Ok(test)
    }
}
pub struct meterpreter {
    pub sessionid:String,
    pub client:Client,
}
impl meterpreter {
    pub fn new(client:Client,sessionid:String) -> Self {
        meterpreter {
            sessionid:sessionid,
            client:client,
        }
    }
    pub fn write(&self,data:String) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn read(&self) -> Result<String,MsfError> {
        let test:String=String::new();
        Ok(test)
    }
    pub fn run_single(&self,command:String) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn script(&self,scriptname:String) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn session_detach(&self) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn session_kill(&self) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn tabs(&self,inputline:String) -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
}
pub fn compactible_modules(client:Client,sessionid:String) -> Result<Vec<String>,MsfError> {
    let test:Vec<String>=Vec::new();
    Ok(test)
}
pub fn shell_upgrade(client:Client,sessionid:String,connecthost:String,connectport:i32) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub trait ring {
    pub fn new(sessionid:String) -> Self {
		sessionid
	}
    pub fn clear(&self) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn last(&self) -> Result<i32,MsfError> {
        let test:i32=1;
        Ok(test)
    }
    pub fn put(&self,data:String) -> Result<i32,MsfError> {
        let test:i32=1;
        Ok(test)
    }
}
