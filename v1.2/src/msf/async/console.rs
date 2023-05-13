#![allow(non_snake_case)]
use crate::error::Error as E;
use crate::client::Client;
#[path="../blocking/console.rs"] mod console;
use serde::de::DeserializeOwned as DOwned;

pub async fn create<T:DOwned>(client:Client) -> Result<T,E> {
    console::create(client.clone())
}
pub async fn destroy<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    console::destroy(client.clone(),consoleID)
}
"pub async fn list<T:DOwned>(client:Client) -> Result<T,E> {
	console::list(client.clone())
}
"pub async fn write<T:DOwned>(client:Client,consoleID:&str,command:&str) -> Result<T,E> {
    console::write(client.clone(),consoleID,command)
}
pub async fn read<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    console::read(client.clone(),consoleID)
}
pub async fn detach_session<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    console::detach_session(client.clone(),consoleID)
}
pub async fn kill_session<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    console::kill_session(client.clone(),consoleID)
}
pub async fn tabs<T:DOwned>(client:Client,consoleID:&str,inputlinestr:&str) -> Result<T,E> {
    console::tabs(client.clone(),consoleID,inputlinestr)
}
