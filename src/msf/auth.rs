#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
use error::MsfError;
use structs::auth;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

pub fn logout(client: Client,out_tok:String) -> Result<bool,MsfError> {
    let tes:bool=true;
    Ok(tes)
}
pub fn token_add(client:Client,new_tok:String) -> Result<bool,MsfError> {
    let tes:bool=true;
    Ok(tes)
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
