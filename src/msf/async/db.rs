use crate::{error::Error as E,client::Client};
#[path="../blocking/db.rs"] mod db;
use serde::de::DeserializeOwned as DOwned;
use std::fs::File;
use std::collections::HashMap;

pub async fn hosts<T:DOwned>(client:Client) -> Result<T,E> {
    db::hosts(client)
}
pub async fn get_host<T:DOwned>(client:Client,workspace:Option<String>,host:&str) -> Result<T,E> {
    db::get_host(client,workspace,host)
}
pub async fn report_host<T:DOwned>(client:Client,workspace:Option<String>,host:&str) -> Result<T,E> {
    db::report_host(client,workspace,host)
}
pub async fn del_host<T:DOwned>(client:Client,workspace:Option<String>,host:&str) -> Result<T,E> {
    db::del_host(client,workspace,host)
}

pub async fn services<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::services(client,hash)
}
pub async fn report_service<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::report_service(client,hash)
}
pub async fn get_service<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::get_service(client,hash)
}
pub async fn del_service<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::del_service(client,hash)
}

pub async fn vulns<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::vulns(client,hash)
}
pub async fn del_vuln<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::del_vuln(client,hash)
}
pub async fn report_vuln<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::report_vuln(client,hash)
}
pub async fn get_vuln<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::get_vuln(client,hash)
}

pub async fn workspaces<T:DOwned>(client:Client) -> Result<T,E> {
    db::workspaces(client)
}
pub async fn current_workspace<T:DOwned>(client:Client) -> Result<T,E> {
    db::current_workspace(client)
}
pub async fn get_workspace<T:DOwned>(client:Client,workspace:&str) -> Result<T,E> {
    db::get_workspace(client,workspace)
}
pub async fn set_workspace<T:DOwned>(client:Client,workspace:&str) -> Result<T,E> {
    db::set_workspace(client,workspace)
}
pub async fn del_workspace<T:DOwned>(client:Client,workspace:&str) -> Result<T,E>  {
    db::del_workspace(client,workspace)
}
pub async fn add_workspace<T:DOwned>(client:Client,workspace:&str) -> Result<T,E>  {
    db::add_workspace(client,workspace)
}

pub async fn get_note<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::get_note(client,hash)
}
pub async fn report_note<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::report_note(client,hash)
}
pub async fn notes<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::notes(client,hash)
}
pub async fn del_note<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::del_note(client,hash)
}

pub async fn get_client<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::get_client(client,hash)
}
pub async fn clients<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::clients(client,hash)
}
pub async fn del_client<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::del_client(client,hash)
}
pub async fn report_client<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::report_client(client,hash)
}

pub async fn get_ref<T:DOwned>(client:Client,ref_name:&str) -> Result<T,E> {
    db::get_ref(client,ref_name)
}

pub async fn events<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::events(client,hash)
}
pub async fn report_event<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::report_event(client,hash)
}

pub async fn report_loot<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::report_loot(client,hash)
}
pub async fn loots<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::loots(client,hash)
}

pub async fn creds<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    db::creds(client,hash)
}

pub async fn import_data<T:DOwned>(client:Client,data:&str) -> Result<T,E> {
    db::import_data(client,data)
}
pub async fn import_file<T:DOwned>(client:Client,file:File) -> Result<T,E> {
    db::import_file(client,file)
}

pub async fn set_driver<T:DOwned>(client:Client,driver:&str) -> Result<T,E> {
    db::set_driver(client,driver)
}
pub async fn get_driver<T:DOwned>(client:Client) -> Result<T,E> {
    db::get_driver(client)
}

pub async fn dbconnect<T:DOwned>(client:Client,driver:&str,hash:HashMap<String,String>) -> Result<T,E> {
    db::dbconnect(client,driver,hash)
}

pub async fn status<T:DOwned>(client:Client) -> Result<T,E> {
    db::status(client)
}

pub async fn disconnect<T:DOwned>(client:Client) -> Result<T,E> {
    db::disconnect(client)
}
