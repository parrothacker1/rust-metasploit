#![allow(non_camel_case_types)]
use serde::Deserialize as des;
#[derive(des,Debug)]
pub struct exploits {
    pub modules:Vec<String>,
}
#[derive(des,Debug)]
pub struct auxiliary {
    pub modules:Vec<String>,
}
#[derive(des,Debug)]
pub struct post {
    pub modules:Vec<String>,
}
#[derive(des,Debug)]
pub struct payloads {
    pub modules:Vec<String>,
}
#[derive(des,Debug)]
pub struct encoders {
    pub modules:Vec<String>,
}
#[derive(des,Debug)]
pub struct nops {
    pub modules:Vec<String>,
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
pub struct compactible_payloads {
    pub payloads:Vec<String>,
}
#[derive(des,Debug)]
pub struct target_compactible_payloads {
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
    pub job_id:u32,
}
#[derive(des,Debug)]
pub struct execute_payloads {
    pub payload:String,
}
