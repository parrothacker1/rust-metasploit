#[path="../structs/mod.rs"] mod structs;
#[path="../connect.rs"] mod connect;
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
use common::{MsfError,Return_Type};
use error::conerr;
use structs::console;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
pub fn create(client:Client) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
        Ok(val) => {
            if val.get("result").unwrap()=="success" {
                test=Return_Type::Bool(true);
            } else {
                let ret=MsfError {
                    error:true,
                    error_class:val.get("error_class").unwrap().to_string(),
                    error_message:val.get("error_message").unwrap().to_string(),
                };
                test=Return_Type::MsfErr(ret);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn destroy(client:Client,consoleid:String) -> Return_Type {
    let test;
    let conn=connect::connect(client.url);
    match conn {
        Ok(val) => {
            if val.get("result").unwrap()=="success" {
                test=Return_Type::Bool(true);
            } else {
                let ret=MsfError {
                    error:true,
                    error_class:val.get("error_class").unwrap().to_string(),
                    error_message:val.get("error_message").unwrap().to_string(),
                };
                test=Return_Type::MsfErr(ret);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn write(client:Client,consoleid:String,data:String) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
        Ok(val) => {
            if val.get("wrote") == None {
                let ret=MsfError {
                    error:true,
                    error_class:val.get("error_class").unwrap().to_string(),
                    error_message:val.get("error_message").unwrap().to_string(),
                };
                test=Return_Type::MsfErr(ret);
            } else {
                let wrote_string:String=val.get("wrote").unwrap().to_string();
                let wrote:i32=wrote_string.parse().unwrap();
                test=Return_Type::Int(wrote);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn read(client:Client,consoleid:String) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
        Ok(val) => {
            if val.get("data")==None {
                let ret=MsfError {
        },
        Err(_e) => {
            tes=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn session_detach(client:Client,consoleid:String) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub fn session_kill(client:Client,consoleid:String) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub fn tabs(client:Client,consoleid:String,inputline:String) -> Result<Vec<String>,MsfError> {
    let test=Vec::new();
    Ok(test)
}
