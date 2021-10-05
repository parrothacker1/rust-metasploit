#![allow(non_camel_case_types)]
use serde::Deserialize as des;

#[derive(des,Debug,Clone)]
pub struct hosts {
    pub hosts:Vec<String>,
    pub created_at:i32,
    pub address:String,
    pub mac:String,
    pub name:String,
    pub state:String,
    pub os_name:String,
    pub os_flavor:String,
    pub os_sp:String,
    pub os_lang:String,
    pub updated_list:i32,
    pub purpose:String,
    pub info:String
}
#[derive(des,Debug,Clone)]
pub struct add_workspace {
    pub result:String,
}
