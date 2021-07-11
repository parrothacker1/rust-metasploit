#![allow(non_camel_case_types)]
use serde::Deserialize as des;
#[derive(des,Debug)]
pub struct login {
    pub result:String,
    pub token:String,
}
#[derive(des,Debug)]
pub struct logout {
    pub result:String,
}
#[derive(des,Debug)]
pub struct tokenadd {
    pub result:String,
}
#[derive(des,Debug)]
pub struct tokengen {
    pub result:String,
    pub token:String,
}
#[derive(des,Debug)]
pub struct tokenlist {
    pub tokens:Vec<String>,
}
#[derive(des,Debug)]
pub struct tokenrem {
    pub result:String,
}
