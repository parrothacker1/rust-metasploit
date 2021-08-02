#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use common::{corelist,version,MsfError,modules,ReturnValue as Return_Type};
use error::conerr;
use serde::Serialize as se;
use rmp_serde::Serializer;
use serde_json::{self,from_value};
use std::collections::HashMap;
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
#[derive(se)]
struct modulestruct(String,String);
#[derive(se)]
struct addmodule(String,String,String);
pub fn add_module_path(client:Client,path:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=addmodule("core.add_module_path".to_string(),client.token.unwrap(),path);
    byte.serialize(&mut serializer);
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
			let ret:Result<modules,serde_json::Error>=from_value(val);
			match ret {
				Ok(retn) => {
					test=Return_Type::CoreModules(retn);
				},
				Err(_e) => {
					let retn:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(retn);
				},
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
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=modulestruct("core.module_status".to_string(),client.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
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
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=modulestruct("core.reload_modules".to_string(),client.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
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
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=modulestruct("core.save".to_string(),client.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
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
#[derive(se)]
struct coresetg(String,String,String,String);
pub fn setg(client:Client,name:String,value:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=coresetg("core.setg".to_string(),client.token.unwrap(),name,value);
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
        Err(_) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
#[derive(se)]
struct coreunsetg(String,String,String);
pub fn unsetg(client:Client,name:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=coreunsetg("core.unsetg".to_string(),client.token.unwrap(),name);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				test=Return_Type::Bool(true);
			} else {
				let ret:MsfError=from_value(val).unwrap();
				test=Return_Type::MsfErr(ret);
			};
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}

pub fn thread_list(client:Client) -> Return_Type{
	let test;
	let mut body=Vec::new();
	let mut serializer=Serializer::new(&mut body);
	let byte=modulestruct("core.thread_list".to_string(),client.token.unwrap());
	byte.serialize(&mut serializer).unwrap();
	let con=connect::connect(client.url,body);
	match con {
		Ok(val) => {
			let ret:Result<HashMap<i32,corelist>,serde_json::Error>=from_value(val);
			match ret {
				Ok(retn) => {
					test=Return_Type::CoreList(retn);
				},
				Err(_e) => {
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
struct threadkill(String,String,i32);
pub fn thread_kill(client:Client,threadID:i32) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=threadkill("core.thread_kill".to_string(),client.token.unwrap(),threadID);
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
pub fn version(client:Client) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=modulestruct("core.version".to_string(),client.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
			let ret:Result<version,serde_json::Error>=from_value(val);
			match ret {
				Ok(ret) => {
					test=Return_Type::CoreVersion(ret);
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
pub fn stop(client:Client) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=modulestruct("core.stop".to_string(),client.token.unwrap());
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
