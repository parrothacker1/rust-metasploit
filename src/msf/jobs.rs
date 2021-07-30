#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
#[path="./common.rs"] mod common;
use connect::Parse_Type as PType;
use common::{MsfError,Return_Type,jobinfo};
use error::conerr;
use serde_json::{self,from_value};
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn list(client.Client) -> Return_Type {
    let test;
    let body=vec![PType::String("job.list".to_string()),PType::String(client.token.unwrap())];
    let con=connect::connect(client.url,body);

}
pub fn info(client:Client,jobid:String) -> Return_Type {
    let test;
    let body=vec![PType::String("job.info".to_string()),PType::String(client.token.unwrap()),PType::String(jobid)];
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
			let ret:Result<jobinfo,serde_json::Error>=from_value(val);
			match ret {
				Ok(retn) => {
					test=Return_Type::JobInfo(retn);
				},
				Err(_e) => {
					let ret:MsfError=from_value(val).unwrap();
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
    let body=vec![PType::String("job.stop".to_string()),PType::String(jobid)];
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
