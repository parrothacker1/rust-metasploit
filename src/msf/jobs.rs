#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
#[path="./common.rs"] mod common;
use common::{MsfError,ReturnValue as Return_Type,jobinfo};
use error::conerr;
use serde::Serialize as se;
use rmp_serde::Serializer;
use std::collections::HashMap;
use serde_json::{self,from_value};
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
#[derive(se)]
struct jobstruct(String,String);
pub fn list(client:Client) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=jobstruct("job.list".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url.clone(),body);
    match con {
		Ok(val) => {
			let ret:Result<HashMap<String,String>,serde_json::Error>=from_value(val.clone());
			match ret {
				Ok(ret) => {
					test=Return_Type::JobList(ret);
				},
				Err(_e) => {
					let ret:MsfError=from_value(val.clone()).unwrap();
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
struct jobinfostruct(String,String,String);
pub fn info(client:Client,jobid:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=jobinfostruct("job.info".to_string(),client.token.as_ref().unwrap().to_string(),jobid.to_string());
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url.clone(),body);
    match con {
		Ok(val) => {
			let ret:Result<jobinfo,serde_json::Error>=from_value(val.clone());
			match ret {
				Ok(retn) => {
					test=Return_Type::JobInfo(retn);
				},
				Err(_e) => {
					let ret:MsfError=from_value(val.clone()).unwrap();
					test=Return_Type::MsfErr(ret);
				},
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string())
		},
	}
    test
}
pub fn stop(client:Client,jobid:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=jobinfostruct("job.stop".to_string(),client.token.as_ref().unwrap().to_string(),jobid.clone());
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
		Err(_e)=> {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
