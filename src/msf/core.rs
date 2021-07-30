#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use common::{MsfError,modules,Return_Type};
use error::conerr;
use connect::Parse_Type as PType;
use serde_json::from_value;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn add_module_path(client:Client,path:String) -> Return_Type {
    let test;
    let body=vec![PType::String("core.add_module_path".to_string()),PType::String(client.token.unwrap()),PType::String(path)];
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
            if val.get("exploits")==None {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                let ret:modules=from_value(val).unwrap();
                test=Return_Type::CoreModules(ret);
            }
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
    }
    test
}
pub fn module_stats(client:Client) -> Return_Type {
    let test;
    let body=vec![PType::String("core.module_status".to_string()),PType::String(client.token.unwrap())];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("exploits")==None {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                let ret:modules=from_value(val).unwrap();
                test=Return_Type::CoreModules(ret);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn reload_modules(client:Client) -> Return_Type {
    let test;
    let body=vec![PType::String("core.relaod_modules".to_string()),PType::String(client.token.unwrap())];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("exploits")==None {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                let ret:modules=from_value(val).unwrap();
                test=Return_Type::CoreModules(ret);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn save(client:Client) -> Return_Type {
    let test;
    let body=vec![PType::String("core.save".to_string()),PType::String(client.token.unwrap())];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("result").unwrap().as_str().unwrap()=="sucess" {
                test=Return_Type::Bool(true);
            } else {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            }
        },
        Err(_) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn setg(client:Client,name:String,value:String) -> Return_Type {
    let test;
    let body=vec![PType::String("core.setg".to_string()),PType::String(client.token.unwrap()),PType::String(name),PType::String(value)];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("result").unwrap().as_str().unwrap()=="success" {
                test=Return_Type::Bool(true);
            } else {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            }
        },
        Err(_) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
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
