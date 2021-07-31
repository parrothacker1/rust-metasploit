#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use common::{corelist,version,MsfError,modules,ReturnValue as Return_Type};
use error::conerr;
use connect::Parse_Type as PType;
use serde_json::{self,from_value};
use std::collections::HashMap;
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
pub fn unsetg(client:Client,name:String) -> Return_Type {
    let test;
    let body=vec![PType::String("core.unsetg".to_string()),PType::String(client.token.unwrap()),PType::String(name)];
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
	let body=vec![PType::String("core.thread_list".to_string()),PType::String(client.token.unwrap())];
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
pub fn thread_kill(client:Client,threadID:i32) -> Return_Type {
    let test;
    let body=vec![PType::String("core.thread_kill".to_string()),PType::String(client.token.unwrap()),PType::Int(threadID)];
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
    let body=vec![PType::String("core.version".to_string()),PType::String(client.token.unwrap())];
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
    let body=vec![PType::String("core.stop".to_string()),PType::String(client.token.unwrap())];
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
