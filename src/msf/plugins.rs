#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
#[path="./common.rs"] mod common;
use common::{MsfError,ReturnValue as Return_Type,pluginloaded};
use serde::Serialize as se;
use rmp_serde::Serializer;
use std::collections::HashMap;
use error::conerr;
use serde_json::from_value;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
#[derive(se)]
struct pluginload(String,String,String,HashMap<String,String>);
pub fn load(client:Client,pluginname:String,options:HashMap<String,String>) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=pluginload("plugin.load".to_string(),client.token.unwrap(),pluginname,options);
    byte.serialize(&mut serializer).unwrap();
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
#[derive(se)]
struct pluginuload(String,String,String);
pub fn unload(client:Client,pluginname:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=pluginuload("plugin.unload".to_string(),client.token.unwrap(),pluginname);
    byte.serialize(&mut serializer).unwrap();
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
#[derive(se)]
struct pluginloadedstruct(String,String);
pub fn loaded(client:Client) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=pluginloadedstruct("plugin.loaded".to_string(),client.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("plugins") == None {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                let ret:pluginloaded=from_value(val).unwrap();
                test=Return_Type::ArrayStr(ret.plugins);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}

