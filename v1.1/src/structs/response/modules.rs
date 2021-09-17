#![allow(non_camel_case_types)]
use serde::Deserialize as des;
use crate::value::Value;
#[derive(des,Debug)]
pub struct list {
    pub modules:Vec<String>,
}
#[derive(des,Debug,Clone)]
pub struct list_encode_formats(pub Vec<String>);

#[derive(des,Debug,Clone)]
pub struct search {
    pub r#type:String,
    pub name:String,
    pub fullname:String,
    pub disclosuredate:String,
}

#[derive(des,Debug,Clone)]
pub struct check {
    pub job_id:String,
}

#[derive(des,Debug,Clone)]
pub struct info {
    pub name:String,
    pub description:String,
    pub license:Vec<String>,
    pub filepath:String,
    pub version:Option<String>,
    pub rank:String,
    pub authors:Vec<String>,
    pub references:Vec<String>,
}
#[derive(des,Debug,Clone)]
pub struct  options {
    pub r#type:String,
    pub required:bool,
    pub advanced:bool,
    pub evasion:bool,
    pub desc:String,
    pub default:Option<Value>,
    pub enums:Option<Vec<String>>,
}
#[derive(des,Debug)]
pub struct compactible_payloads {
    pub payloads:Vec<String>,
}
#[derive(des,Debug)]
pub struct compactible_sessions {
    pub sessions:Vec<String>,
}
#[derive(des,Debug)]
pub struct encode {
    pub encoded:String,
}
#[derive(des,Debug)]
pub struct execute_non_payloads {
    pub job_id:i32,
}
#[derive(des,Debug)]
pub struct execute_payloads {
    pub payload:Value,
}
