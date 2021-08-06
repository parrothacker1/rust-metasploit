#![allow(non_camel_case_types)]
use serde::Deserialize as des;
#[derive(des,Debug)]
pub struct addmodpath {
    pub exploits:u32,
    pub auxiliary:u32,
    pub post:u32,
    pub encoders:u32,
    pub nops:u32,
    pub payloads:u32,
}
#[derive(des,Debug)]
pub struct modulestat {
    pub exploits:u32,
    pub auxiliary:u32,
    pub post:u32,
    pub encoders:u32,
    pub nops:u32,
    pub payloads:u32,
}
#[derive(des,Debug)]
pub struct reloadmod {
    pub exploits:u32,
    pub auxiliary:u32,
    pub post:u32,
    pub encoders:u32,
    pub nops:u32,
    pub payloads:u32,
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
