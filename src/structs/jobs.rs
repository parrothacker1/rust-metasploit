#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use serde::Deserialize as des;

#[derive(des,Debug)]
pub struct Data {
    pub EnableContextEncoding:bool,
    pub DisablePayloadHandler:bool,
    pub SSL:bool,
    pub SSLVersion:String,
    pub PAYLOAD:String, 
}
#[derive(des,Debug)]
pub struct info {
    pub jid:u32,
    pub name:String,
    pub start_time:u32,
    pub uripath:String,
    pub datastore:Data,

}
#[derive(des,Debug)]
pub struct stop {
    pub result:String,
}
