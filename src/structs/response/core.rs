#![allow(non_camel_case_types)]
use serde::Deserialize as des;
#[derive(des,Debug,Clone)]
pub struct addmodpath {
    pub exploits:i32,
    pub auxiliary:i32,
    pub post:i32,
    pub encoders:i32,
    pub nops:i32,
    pub payloads:i32,
}
#[derive(des,Debug,Clone)]
pub struct modulestat {
    pub exploits:i32,
    pub auxiliary:i32,
    pub post:i32,
    pub encoders:i32,
    pub nops:i32,
    pub payloads:i32,
}
#[derive(des,Debug,Clone)]
pub struct reloadmod {
    pub exploits:i32,
    pub auxiliary:i32,
    pub post:i32,
    pub encoders:i32,
    pub nops:i32,
    pub payloads:i32,
}
#[derive(des,Debug)]
pub struct save {
    pub result:String,
}
#[derive(des,Debug)]
pub struct setg {
    pub result:String,
}
#[derive(des,Debug)]
pub struct unsetg {
    pub result:String,
}
#[derive(des,Debug)]
pub struct threadlist {
	pub status:String,
	pub critical:bool,
	pub name:String,
	pub started:String,
}
#[derive(des,Debug)]
pub struct threadkill {
    pub result:String,
}
#[derive(des,Debug)]
pub struct version {
    pub version:String,
    pub ruby:String,
    pub api:String,
}
#[derive(des,Debug)]
pub struct stop {
    pub result:String,
}
