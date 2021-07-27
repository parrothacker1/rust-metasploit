#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
use error::MsfError;
use structs::auth;
use serde_json::Value;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn logout(client: Client,out_tok:String) -> Result<bool,MsfError> {
    let tes:bool;
    let ret=connect::connect(client.url);
    match ret {
        Ok(retu) => {

}
pub fn token_add(client:Client,new_tok:String) -> Result<bool,MsfError> {
    let tes:bool;
    let ret:Value=connect::connect(client.url).unwrap();
    if ret.get("result").unwrap()==&Value::from(String::from("success")) {
        tes=true;
    } else {
        tes=false;
    }
}
pub fn token_gen(client:Client) -> Result<String,MsfError> {
    let tes=String::new();
    Ok(tes)
}
pub fn token_list(client:Client) -> Result<Vec<String>,MsfError> {
    let tes:Vec<String>=Vec::new();
    Ok(tes)
}
pub fn token_remove(client:Client,token_rem:String) -> Result<bool,MsfError> {
    let tes:bool=true;
    Ok(tes)
}
