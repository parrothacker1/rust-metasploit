use crate::client::Client;
use std::collections::HashMap;
use crate::error::Error as E;
#[path="../blocking/plugins.rs"] mod plugins;
use serde::de::DeserializeOwned as DOwned;

pub async fn load<T:DOwned>(client:Client,pluginnamestr:&str,options:HashMap<String,String>) -> Result<T,E> {
    plugins::load(client.clone(),pluginnamestr,options)
}
pub async fn unload<T:DOwned>(client:Client,pluginnamestr:&str) -> Result<T,E> {
    plugins::unload(client.clone(),pluginnamestr)
}
pub async fn list<T:DOwned>(client:Client) -> Result<T,E> {
    plugins::list(client.clone())
}
