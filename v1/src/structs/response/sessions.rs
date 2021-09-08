#![allow(non_camel_case_types)]
#![allow(dead_code)]
use serde::Deserialize as des;
use crate::value::Value;
use std::collections::HashMap;

pub type list=HashMap<i32,HashMap<String,Value>>;
#[derive(des,Debug)]
pub struct stop {
    pub result:String,
}
#[derive(des,Debug,Clone)]
pub struct shell_read {
    pub seq:i32,
    pub data:String,
}
#[derive(des,Debug)]
pub struct shell_write {
    pub write_count:String,
}
#[derive(des,Debug,Clone)]
pub struct meterpreter_write {
    pub result:String,
}
#[derive(des,Debug)]
pub struct meterpreter_read {
    pub data:String,
}
#[derive(des,Debug)]
pub struct meterpreter_run_single {
    pub result:String,
}
#[derive(des,Debug)]
pub struct meterpreter_script {
    pub result:String,
}
#[derive(des,Debug)]
pub struct meterpreter_session_detach {
    pub result:String,
}
#[derive(des,Debug)]
pub struct meterpreter_session_kill {
    pub result:String,
}
#[derive(des,Debug)]
pub struct meterpreter_tabs {
    pub tabs:Vec<String>,
}
#[derive(des,Debug)]
pub struct compactible_modules {
    pub modules:Vec<String>,
}
#[derive(des,Debug)]
pub struct shell_upgrade {
    pub result:String,
}
#[derive(des,Debug)]
pub struct ring_clear {
    pub result:String,
}
#[derive(des,Debug)]
pub struct ring_last {
    pub seq:i32,
}
#[derive(des,Debug)]
pub struct ring_put {
    pub write_count:String,
}
