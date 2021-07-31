#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
use common::{MsfError,ReturnValue as Return_Type};
use connect::Parse_Type as PType;
use std::collections::HashMap;
use error::conerr;
use serde_json::from_value;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
pub fn load(client:Client,pluginname:String,options:HashMap<String,String>) -> Result<bool,MsfError> {
    let test;
    let body=vec![PType::String("plugin.load".to_string()),PType::String(client.token.unwrap()),PType::String(pluginname),PType::HashMapStr(options)];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("result")==None {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                if val.get("result").unwrap().as_str().unwrap() == "success" {
                    test=Return_Type::Bool(true);
                } else {
                    test=Return_Type::Bool(false);
                }
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn unload(client:Client,pluginname:String) -> Return_Type {
    let test;
    let body=vec![PType::String("plugin.unload".to_string()),PType::String(client.token.unwrap()),PType::String(pluginname)];
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("result")==None {
                let ret:MsfError = from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                if val.get("result").unwrap().as_str().unwrap() == "success" {
                    test=Return_Type::Bool(true);
                } else {
                    test=Return_Type::Bool(false);
                }
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn plugins(client:Client) -> Result<Vec<String>,MsfError> {
    let test:Vec<String>=Vec::new();
    Ok(test)
}

