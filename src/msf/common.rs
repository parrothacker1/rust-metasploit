#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use serde::Deserialize as des;
use std::collections::HashMap;

#[derive(des,Debug)]
pub struct MsfError {
    error:bool,
    error_class:String,
    error_message:String,
}
pub enum ReturnValue {
	Bool(bool),
	String(String),
    Int(i64),
	MsfErr(MsfError),
	ArrayStr(Vec<String>),
	ArrayInt(Vec<i32>),
    ConsoleCreate(create),
    ConsoleRead(read),
    ConsoleList(HashMap<String,consolelist>),
    CoreList(HashMap<i32,corelist>),
    CoreModules(modules),
    CoreVersion(version),
    JobInfo(jobinfo),
    JobList(HashMap<String,String>),
    ModuleInfo(info),
    ModuleOption(HashMap<String,moduleoption>),
}
#[derive(des,Debug)]
pub struct modulelist {
	pub modules:Vec<String>,
}
#[derive(des,Debug)]
pub struct moduleoption {
	pub r#type:String,
	pub required:bool,
	pub advanced:bool,
	pub evasion:bool,
	pub desc:String,
	pub default:String,
}
#[derive(des,Debug)]
pub struct compactiblepayload {
	pub payloads:Vec<String>,
}
#[derive(des,Debug)]
pub struct compactiblesessions {
	pub sessions:Vec<i32>,
}
#[derive(des,Debug)]
pub struct consoletabs {
	pub tabs:Vec<String>,
}
#[derive(des,Debug)]
pub struct tokenlist {
	pub token_list:Vec<String>,
}
#[derive(des,Debug)]
pub struct create {
    pub id:i64,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug)]
pub struct read {
    pub data:String,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug)]
pub struct modules {
    pub exploits:i64,
    pub auxiliary:i64,
    pub post:i64,
    pub encoders:i64,
    pub nops:i64,
    pub payloads:i64,
}
#[derive(des,Debug)]
pub struct version {
    pub version:String,
    pub ruby:String,
    pub api:String,
}
#[derive(des,Debug)]
pub struct Data {
    pub EnableContextEncoding:bool,
    pub DisablePayloadHandler:bool,
    pub SSL:bool,
    pub SSLVersion:String,
    pub PAYLOAD:String,
}
#[derive(des,Debug)]
pub struct jobinfo {
    pub jid:u32,
    pub name:String,
    pub start_time:u32,
    pub uripath:String,
    pub datastore:Data,

}
#[derive(des,Debug)]
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
#[derive(des,Debug)]
pub struct consolelist {
    pub id:String,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug)]
pub struct corelist {
	pub status:String,
	pub critical:bool,
	pub name:String,
	pub started:String,
}
