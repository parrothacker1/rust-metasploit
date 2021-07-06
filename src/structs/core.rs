use serde::{Deserialize as des,Serialize as ses};

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
pub struct coresave {
    pub result:String,
}
