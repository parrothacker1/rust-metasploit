#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use error::{MsfError,conerr};
use structs::auth;
use common::Return_Type;
use serde_json::Value;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn logout(client: Client,out_tok:String) -> Return_Type {
    let tes;
    let conn=connect::connect(client.url);
    match conn {
		Ok(val) => {
			if val.get("result").unwrap()=="success" {
				tes=Return_Type::Bool(true);
			} else {
				let ret=MsfError {
					error:true,
					error_class:val.get("error_class").unwrap().to_string(),
					error_message:val.get("error_message").unwrap().to_string(),
				};
				tes=Return_Type::MsfErr(ret);
			};
		},
		Err(e) => {
			tes=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
	tes
}
pub fn token_add(client:Client,new_tok:String) -> Return_Type {
    let tes;
    let conn=connect::connect(client.url);
    match conn {
		Ok(val) => {
			if val.get("result").unwrap()=="success" {
				tes=Return_Type::Bool(true);
			} else {
				let ret=MsfError {
					error:true,
					error_class:val.get("error_class").unwrap().to_string(),
					error_message:val.get("error_message").unwrap().to_string(),
				};
				tes=Return_Type::MsfErr(ret);
			};
		},
		Err(_e) => {
			tes=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    tes
}
pub fn token_gen(client:Client) -> Return_Type {
    let tes;
    let conn=connect::connect(client.url);
    match conn {
		Ok(val) => {
			if val.get("result").unwrap()=="success" {
				tes=Return_Type::String(val.get("token").unwrap().to_string());
			} else {
				let ret=MsfError {
					error:true,
					error_class:val.get("error_class").unwrap().to_string(),
					error_message:val.get("error_message").unwrap().to_string(),
				};
				tes=Return_Type::MsfErr(ret);
			};
		},
		Err(_e) => {
			tes=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    tes
}
pub fn token_list(client:Client) -> Return_Type {
    let tes:Vec<String>=Vec::new();
    Ok(tes)
}
pub fn token_remove(client:Client,token_rem:String) -> Result<bool,conerr> {
    let tes:bool=true;
    Ok(tes)
}
