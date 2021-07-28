#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use error::conerr;
use common::{MsfError,Return_Type};
use serde_json::value::Value;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn logout(client: Client,out_tok:String) -> Return_Type {
    let tes;
    let conn=connect::connect(client.url);
    match conn {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				tes=Return_Type::Bool(true);
			} else {
				let ret=MsfError {
					error:val.get("error").unwrap().as_bool().unwrap(),
					error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
					error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
				};
				tes=Return_Type::MsfErr(ret);
			};
		},
		Err(e) => {
			tes=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	};
	tes
}
pub fn token_add(client:Client,new_tok:String) -> Return_Type {
    let tes;
    let conn=connect::connect(client.url);
    match conn {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				tes=Return_Type::Bool(true);
			} else {
				let ret=MsfError {
					error:val.get("error").unwrap().as_bool().unwrap(),
					error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
					error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
				};
				tes=Return_Type::MsfErr(ret);
			};
		},
		Err(_e) => {
			tes=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	};
    tes
}
pub fn token_gen(client:Client) -> Return_Type {
    let tes;
    let conn=connect::connect(client.url);
    match conn {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				tes=Return_Type::String(val.get("token").unwrap().to_string());
			} else {
				let ret=MsfError {
					error:val.get("error").unwrap().as_bool().unwrap(),
					error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
					error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
				};
				tes=Return_Type::MsfErr(ret);
			};
		},
		Err(_e) => {
			tes=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	};
    tes
}
pub fn token_list(client:Client) -> Return_Type {
    let tes;
    let mut ret;
    let con=connect::connect(client.url);
    match con {
        Ok(val) => {
            if val.get("token_list")==None {
                let ret=MsfError {
                    error:val.get("error").unwrap().as_bool().unwrap(),
                    error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
                    error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
                };
                tes=Return_Type::MsfErr(ret);
            } else {
                ret=val.get("token_list").unwrap().as_array().unwrap().to_vec(); 
                tes=Return_Type::Array(ret);
            };
        },
        Err(_e) => {
            tes=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    };
    tes
}
pub fn token_remove(client:Client,token_rem:String) -> Return_Type {
    let tes;
    let con=connect::connect(client.url);
    match con {
        Ok(val) => {
            if val.get("result").unwrap().as_str().unwrap()=="success" {
                tes=Return_Type::Bool(true);
            } else {
                let ret=MsfError {
                    error:val.get("error").unwrap().as_bool().unwrap(),
                    error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
                    error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
                };
                tes=Return_Type::MsfErr(ret);
            };
        },
        Err(_e) => {
            tes=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    };
    tes
}
