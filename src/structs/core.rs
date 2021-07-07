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
pub struct coresave {
    pub result:String,
}
#[derive(des,Debug)]
pub struct coresetg {
    pub result:String,
}
#[derive(des,Debug)]
pub struct coreunsetg {
    pub result:String,
}
#[derive(des,Debug)]
pub struct corethreadkill {
    pub result:String,
}
#[derive(des,Debug)]
pub struct coreversion {
    pub version:String,
    pub ruby:String,
    pub api:String,
}
#[derive(des,Debug)]
pub struct corestop {
    pub result:String,
}
