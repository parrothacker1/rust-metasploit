#![allow(non_camel_case_types)]
use serde::Deserialize as des;
#[derive(des,Debug,Clone)]
pub struct create {
    pub id:String,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug)]
pub struct destroy {
    pub result:String,
}
#[derive(des,Debug,Clone)]
pub struct listcomponents {
    pub id:String,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug,Clone)]
pub struct list {
	pub consoles:Vec<listcomponents>,
}
#[derive(des,Debug)]
pub struct write {
    pub wrote:i32,
}
#[derive(des,Debug,Clone)]
pub struct read {
    pub data:String,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug)]
pub struct session_detach {
    pub result:String,
}
#[derive(des,Debug)]
pub struct session_kill {
    pub result:String,
}
#[derive(des,Debug)]
pub struct tabs {
    pub tabs:Vec<String>,
}
