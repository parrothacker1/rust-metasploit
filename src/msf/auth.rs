#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use error::conerr;
use rmp_serde::Serializer;
use serde::Serialize as se;
use common::{MsfError,ReturnValue as Return_Type,tokenlist};
use serde_json::value::{from_value};

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
#[derive(se)]
struct logoutstruct(String,String,String);
pub fn logout(client: Client,out_tok:String) -> Return_Type {
    let tes;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=logoutstruct("auth.logout".to_string(),client.token.unwrap(),out_tok);
    byte.serialize(&mut serializer).unwrap();
    let conn=connect::connect(client.url,body);
    match conn {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				tes=Return_Type::Bool(true);
			} else {
				let ret:MsfError=from_value(val).unwrap();
				tes=Return_Type::MsfErr(ret);
			};
		},
		Err(e) => {
			tes=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	};
	tes
}
#[derive(se)]
struct tokenadd(String,String,String);
pub fn token_add(client:Client,new_tok:String) -> Return_Type {
    let tes;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=tokenadd("auth.token_add".to_string(),client.token.unwrap(),new_tok);
    byte.serialize(&mut serializer).unwrap();
    let conn=connect::connect(client.url,body);
    match conn {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				tes=Return_Type::Bool(true);
			} else {
				let ret:MsfError=from_value(val).unwrap();
				tes=Return_Type::MsfErr(ret);
			};
		},
		Err(_e) => {
			tes=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	};
    tes
}
#[derive(se)]
struct tokengen(String,String);
pub fn token_gen(client:Client) -> Return_Type {
    let tes;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=tokengen("auth.token_generate".to_string(),client.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
    let conn=connect::connect(client.url,body);
    match conn {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				tes=Return_Type::String(val.get("token").unwrap().to_string());
			} else {
				let ret:MsfError=from_value(val).unwrap();
				tes=Return_Type::MsfErr(ret);
			};
		},
		Err(_e) => {
			tes=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	};
    tes
}
#[derive(se)]
struct tokenliststruct(String,String);
pub fn token_list(client:Client) -> Return_Type {
    let tes;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=tokenliststruct("auth.token_list".to_string(),client.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("token_list")==None {
                let ret:MsfError=from_value(val).unwrap();
                tes=Return_Type::MsfErr(ret);
            } else {
                let ret:tokenlist=from_value(val).unwrap();
                tes=Return_Type::ArrayStr(ret.token_list);
            };
        },
        Err(_e) => {
            tes=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    };
    tes
}
#[derive(se)]
struct tokenremove(String,String,String);
pub fn token_remove(client:Client,token_rem:String) -> Return_Type {
    let tes;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=tokenremove("auth.token_remove".to_string(),client.token.unwrap(),token_rem);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("result").unwrap().as_str().unwrap()=="success" {
                tes=Return_Type::Bool(true);
            } else {
                let ret:MsfError=from_value(val).unwrap();
                tes=Return_Type::MsfErr(ret);
            };
        },
        Err(_e) => {
            tes=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    };
    tes
}
