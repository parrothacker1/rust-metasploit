use serde::{Serialize as se,Deserialize as de};
use rmp_serde::decode::from_read;
use rmpv::Value;
use rmp_serde::Serializer;
use reqwest::blocking::Client;
use reqwest::header;

#[derive(se)]
struct login(String,String,String);
#[derive(de,Debug)]
struct auth {
    token:String,
}

fn connect(url:String,user:String,passwd:String) -> Vec<u8> {
    let mut buf=vec![];
    let mut body:Vec<u8>=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=login("auth.login".to_string(),user,passwd);
    byte.serialize(&mut serializer).unwrap();
    let mut header=header::HeaderMap::new();
    header.insert(header::CONTENT_TYPE,header::HeaderValue::from_static("binary/message-pack"));
    let client=Client::builder()
        .default_headers(header)
        .danger_accept_invalid_certs(true)
        .build().unwrap();
    let mut reader=client.post(url).body(body).send().unwrap();
    reader.copy_to(&mut buf).unwrap();
    buf
}

fn main() {
    let buf=connect("https://127.0.0.1:4040/api".to_string(),"user".to_string(),"password".to_string());
    let test:auth=from_read(buf.as_slice()).unwrap();
    println!("{:?}",test);
}
