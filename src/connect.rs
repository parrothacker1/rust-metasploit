#![allow(non_camel_case_types)]
use reqwest::blocking::Client;
use reqwest::header;
use std::io::Read;
use serde::Deserialize;
use rmp_serde::{Deserializer,decode::ReadReader};
use std::collections::HashMap;
#[path="./error.rs"] mod error;
use error::ConnectionError as conerr;

pub fn connect(url:String,body:Vec<u8>,buf:&mut Vec<u8>) -> Result<(),conerr>{
	let mut headers=header::HeaderMap::new();
	headers.insert(header::CONTENT_TYPE,header::HeaderValue::from_static("binary/message-pack"));
    let client=Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .build().unwrap();
    let reader=client.post(url).body(body).send()?;
    reader.copy_to(buf);
    Ok(())
}
