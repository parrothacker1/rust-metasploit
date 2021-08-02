#![allow(non_camel_case_types)]
#[path="../connect.rs"] mod connect;
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
use common::{consoletabs,MsfError,create,read,ReturnValue as Return_Type,consolelist};
use error::conerr;
use rmp_serde::Serializer;
use serde::Serialize as se;
use std::collections::HashMap;
use serde_json::{self,from_value};
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
#[derive(se)]
struct createstruct(String,String);
pub fn create(client:Client) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=createstruct("console.create".to_string(),client.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            let ret:Result<create,serde_json::Error>=from_value(val);
            match ret {
                Ok(retv) => {
                    test=Return_Type::ConsoleCreate(retv);
                },
                Err(_) => {
					let ret:MsfError=from_value(val).unwrap();
                    test=Return_Type::MsfErr(ret);
                },
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
#[derive(se)]
struct consoledestroy(String,String,String);
pub fn destroy(client:Client,consoleid:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=consoledestroy("console.destroy".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer).unwrap();
    let conn=connect::connect(client.url,body);
    match conn {
        Ok(val) => {
            if val.get("result") != None && val.get("result").unwrap().as_str().unwrap()=="success" {
                test=Return_Type::Bool(true);
            } else {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
#[derive(se)]
struct consoleliststruct(String,String);
pub fn list(client:Client) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=consoleliststruct("console.list".to_string(),client.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
			let ret:Result<HashMap<String,consolelist>,serde_json::Error>=from_value(val);
			match ret {
				Ok(retn) => {
					test=Return_Type::ConsoleList(retn);
				},
				Err(_e) => {
					let err:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(err);
				},
			}
        },
        Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
#[derive(se)]
struct consolewrite(String,String,String,String);
pub fn write(client:Client,consoleid:String,data:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=consolewrite("console.write".to_string(),client.token.unwrap(),consoleid,data);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("wrote") == None {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                let wrote_string:String=val.get("wrote").unwrap().as_str().unwrap().to_string();
                let wrote:i64=wrote_string.parse().unwrap();
                test=Return_Type::Int(wrote);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
#[derive(se)]
struct consoleread(String,String,String);
pub fn read(client:Client,consoleid:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=consoleread("console.read".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer);
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("data")==None {
                let ret:MsfError=from_value(val).unwrap();
				test=Return_Type::MsfErr(ret);
			} else {
				let ret:read=from_value(val).unwrap();
                test=Return_Type::ConsoleRead(ret);
			};
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
#[derive(se)]
struct consolesession(String,String,String);
pub fn session_detach(client:Client,consoleid:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=consolesession("console.session_detach".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer).unwrap();
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
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
pub fn session_kill(client:Client,consoleid:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=consolesession("console.session_kill".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer);
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
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
#[derive(se)]
struct consoletabstruct(String,String,String,String);
pub fn tabs(client:Client,consoleid:String,inputline:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=consoletabstruct("console.tabs".to_string(),client.token.unwrap(),consoleid,inputline);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
			if val.get("tabs")==None {
				let ret:MsfError=from_value(val).unwrap();
				test=Return_Type::MsfErr(ret);
			} else {
				let ret:consoletabs=from_value(val).unwrap();
				test=Return_Type::ArrayStr(ret.tabs);
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
