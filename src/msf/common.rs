#![allow(non_camel_case_types)]
use serde_json::value::Value;
pub struct MsfError {
    error:bool,
    error_class:String,
    error_message:String,
}
pub enum Return_Type {
	Bool(bool),
	String(String),
    Int(i32),
	MsfErr(MsfError),
	Array(Vec<Value>),
    ConsoleCreate(create),
    ConsoleRead(read),
    CoreModules(modules),
}
#![allow(non_snake_case)]
#[derive(Debug)]
pub struct create {
    pub id:i64,
    pub prompt:String,
    pub busy:bool,
}
#[derive(Debug)]
pub struct read {
    pub data:String,
    pub prompt:String,
    pub busy:bool,
}
#[derive(Debug)]
pub struct modules {
    pub exploits:i64,
    pub auxiliary:i64,
    pub post:i64,
    pub encoders:i64,
    pub nops:i64,
    pub payloads:i64,
}
#[derive(Debug)]
pub struct version {
    pub version:String,
    pub ruby:String,
    pub api:String,
}
#[derive(Debug)]
pub struct Data {
    pub EnableContextEncoding:bool,
    pub DisablePayloadHandler:bool,
    pub SSL:bool,
    pub SSLVersion:String,
    pub PAYLOAD:String, 
}
#[derive(Debug)]
pub struct jobinfo {
    pub jid:u32,
    pub name:String,
    pub start_time:u32,
    pub uripath:String,
    pub datastore:Data,

}
#[derive(Debug)]
pub struct info {
    pub name:String,
    pub description:String,
    pub license:String,
    pub filepath:String,
    pub version:u32,
    pub rank:u32,
    pub authors:Vec<String>,
    pub references:Vec<String>,
}

