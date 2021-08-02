#![allow(non_camel_case_types)]
use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;
use rmp_serde::{encode::to_vec,decode::from_read};
use serde_json::Value;
use std::io::Read;
use std::collections::HashMap;
#[path="./error.rs"] mod error;
use error::ConnectionError as conerr;

pub fn connect(url:String,body:Vec<u8>) -> Result<Value,conerr> {
	let mut headers=header::HeaderMap::new();
	headers.insert(header::CONTENT_TYPE,header::HeaderValue::from_static("binary/message-pack"));
    let client=Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .build().unwrap();
    let reader=client.post(url).body(body).send()?;
    let test:Value=from_read(reader).unwrap();
    Ok(test)
}
