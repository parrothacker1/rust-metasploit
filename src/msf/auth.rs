#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use error::conerr;
use common::{MsfError,Return_Type,tokenlist};
use serde_json::value::{from_value,Value};

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn logout(client: Client,out_tok:String) -> Return_Type {
    let tes;
    let body=vec![connect::Parse_Type::String("auth.login".to_string()),connect::Parse_Type::String(client.token.unwrap()),connect::Parse_Type::String(out_tok)];
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
pub fn token_add(client:Client,new_tok:String) -> Return_Type {
    let tes;
    let body=vec![connect::Parse_Type::String("auth.token_add".to_string()),connect::Parse_Type::String(client.token.unwrap()),connect::Parse_Type::String(new_tok)];
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
pub fn token_gen(client:Client) -> Return_Type {
    let tes;
    let body=vec![connect::Parse_Type::String("auth.token_generate".to_string()),connect::Parse_Type::String(client.token.unwrap())];
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
pub fn token_list(client:Client) -> Return_Type {
    let tes;
    let body=vec![connect::Parse_Type::String("auth.token_list".to_string()),connect::Parse_Type::String(client.token.unwrap())];
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
pub fn token_remove(client:Client,token_rem:String) -> Return_Type {
    let tes;
    let body=vec![connect::Parse_Type::String("auth.token_remove".to_string()),connect::Parse_Type::String(client.token.unwrap()),connect::Parse_Type::String(token_rem)];
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
