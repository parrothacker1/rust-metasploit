#![allow(non_camel_case_types)]
use serde::Deserialize as des;

#[derive(des,Debug)]
pub struct load {
    pub result:String,
}
#[derive(des,Debug)]
pub struct unload {
    pub result:String,
}
#[derive(des,Debug)]
pub struct loaded {
    pub plugins:Vec<String>,
}

