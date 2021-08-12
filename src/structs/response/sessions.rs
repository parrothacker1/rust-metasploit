#![allow(non_camel_case_types)]
use serde::Deserialize as des;
#[derive(des,Debug,Clone)]
pub struct list {
  pub r#type: String,
  pub tunnel_local: String,
  pub tunnel_peer: String,
  pub via_exploit: String,
  pub via_payload:Option<String>,
  pub desc: String,
  pub info: String,
  pub workspace: String,
  pub target_host: String,
  pub username: String,
  pub uuid: String,
  pub exploit_uuid: String,
  pub routes: Option<Vec<String>>,
}
#[derive(des,Debug)]
pub struct stop {
    pub result:String,
}
#[derive(des,Debug,Clone)]
pub struct shell_read {
    pub seq:String,
    pub data:String,
}
#[derive(des,Debug)]
pub struct shell_write {
    pub write_count:i32,
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
