#![allow(non_camel_case_types)]
use serde::Deserialize as des;
#[derive(des,Debug)]
pub struct list {
    pub modules:Vec<String>,
}
#[derive(des,Debug,Clone)]
pub struct info {
    pub name:String,
    pub description:String,
    pub license:String,
    pub filepath:String,
    pub version:i32,
    pub rank:i32,
    pub authors:Vec<String>,
    pub references:Vec<String>,
}
/*#[derive(des,Debug,Clone)]
pub enum options {
    
}*/
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
    pub payload:String,
}
