#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
use crate::client::Client;
use std::collections::HashMap;
use crate::error::Error as E;
use serde::de::DeserializeOwned as DOwned;
#[path="../blocking/modules.rs"] mod modules;

pub struct compactible {
    pub name:String,
    pub client:Client,
}
pub struct list {
    pub client:Client,
}
impl list {
    pub fn new(client:Client) -> Self {
        list {
            client:client,
        }
    }
    pub async fn exploits<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.exploits()
    }
    pub async fn auxiliary<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.auxiliary()
    }
    pub async fn post<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.post()
    }
    pub async fn payloads<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.payloads()
    }
    pub async fn encoders<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.encoders()
    }
    pub async fn nops<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.nops()
    }
}
pub async fn info<T:DOwned>(client:Client,moduletypestr:&str,modulenamestr:&str) -> Result<T,E> {
    modules::info(client.clone(),moduletypestr,modulenamestr)
}
impl compactible {
    pub fn new(modulename:String,client:Client) -> Self {
        compactible {
            name:modulename,
            client:client,
        }
    }
    pub async fn payload<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::compactible::new(self.name.clone(),self.client.clone());
        list.payload()
    }
    pub async fn target_payloads<T:DOwned>(&self,targetindx:i32) -> Result<T,E> {
        let list=modules::compactible::new(self.name.clone(),self.client.clone());
        list.target_payloads(targetindx)
    }
    pub async fn sessions<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::compactible::new(self.name.clone(),self.client.clone());
        list.sessions()
    }
}
pub async fn option<T:DOwned>(client:Client,moduletypestr:&str,modulenamestr:&str) -> Result<T,E> {
    modules::option(client.clone(),moduletypestr,modulenamestr)
}
pub async fn encoder<T:DOwned>(client:Client,datastr:&str,encodermodulestr:&str,options:HashMap<String,String>) -> Result<T,E> {
    modules::encoder(client.clone(),datastr,encodermodulestr,options)
}
pub async fn execute<T:DOwned>(client:Client,moduletypestr:&str,modulenamestr:&str,options:HashMap<String,String>) -> Result<T,E> {
    modules::execute(client.clone(),moduletypestr,modulenamestr,options)
}
