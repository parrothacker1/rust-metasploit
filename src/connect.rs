#![allow(non_camel_case_types)]
use reqwest::blocking::Client;
use std::time::Duration;
use rmp_serde::{encode::to_vec,decode::from_read};
use serde_json::Value;
use std::io::Read;
use std::collections::HashMap;
#[path="./error.rs"] mod error;
use error::ConnectionError as conerr;

pub enum Parse_Type {
    Bool(bool),
    String(String),
    Int(i32),
    HashMapStr(HashMap<String,String>),
}

pub fn connect(url:String,body:Vec<u8>) -> Result<Value,conerr> {
    //let body_con=to_vec(&body);
    let req=Client::builder()
        .default_header(
	let reader=agent.post(&url)
		.set("Content-type","binary/message-pack")
		//.send_bytes()?
        .call()?
		.into_reader();
    let test:Value=from_read(reader).unwrap();
    Ok(test)
}
