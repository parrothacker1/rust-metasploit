#![allow(non_snake_case)]
use crate::error::Error as E;
use crate::client::Client;
#[path="../blocking/core.rs"] mod core;
use serde::de::DeserializeOwned as DOwned;

pub async fn add_module<T:DOwned>(client:Client,pathstr:&str) -> Result<T,E> {
    core::add_module(client.clone(),pathstr)
}
pub async fn module_status<T:DOwned>(client:Client) -> Result<T,E> {
    core::module_status(client.clone())
}
pub async fn reload_module<T:DOwned>(client:Client) -> Result<T,E> {
    core::reload_module(client.clone())
}
pub async fn save<T:DOwned>(client:Client) -> Result<T,E> {
    core::save(client.clone())
}
pub async fn setg<T:DOwned>(client:Client,namestr:&str,valuestr:&str) -> Result<T,E> {
    core::setg(client.clone(),namestr,valuestr)
}
pub async fn unsetg<T:DOwned>(client:Client,namestr:&str) -> Result<T,E> {
    core::unsetg(client.clone(),namestr)
}
pub async fn list_thread<T:DOwned>(client:Client) -> Result<T,E> {
    core::list_thread(client.clone())
}
pub async fn kill_thread<T:DOwned>(client:Client,threadID:i32) -> Result<T,E> {
    core::kill_thread(client.clone(),threadID)
}
pub async fn version<T:DOwned>(client:Client) -> Result<T,E> {
    core::version(client.clone())
}
pub async fn stop<T:DOwned>(client:Client) -> Result<T,E> {
    core::stop(client.clone())
}
