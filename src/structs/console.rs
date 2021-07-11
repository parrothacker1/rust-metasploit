#![allow(non_camel_case_types)]
use serde::Deserialize as des;
#[derive(des,Debug)]
pub struct create {
    pub id:u32,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug)]
pub struct destroy {
    pub result:String,
}
#[derive(des,Debug)]
pub struct write {
    pub wrote:u32,
}
#[derive(des,Debug)]
pub struct read {
    pub data:String,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug)]
pub struct detach {
    pub result:String,
}
#[derive(des,Debug)]
pub struct kill {
    pub result:String,
}
#[derive(des,Debug)]
pub struct tabs {
    pub tabs:Vec<String>,
}
