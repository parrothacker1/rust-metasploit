use crate::client;
#[path="../blocking/auth.rs"] mod auth;
use crate::error::Error as E;
use serde::de::DeserializeOwned as DOwned;

pub async fn logout<T:DOwned>(clientdata:client::Client) -> Result<T,E> {
    auth::logout(clientdata.clone())
}

pub async fn add_token<T:DOwned>(clientdata:client::Client,newtokenstr:&str) -> Result<T,E> {
    auth::add_token(clientdata.clone(),newtokenstr)
}

pub async fn generate_token<T:DOwned>(clientdata:client::Client) -> Result<T,E> {
    auth::generate_token(clientdata.clone())
}

pub async fn list_token<T:DOwned>(clientdata:client::Client) -> Result<T,E> {
    auth::list_token(clientdata.clone())
}

pub async fn remove_token<T:DOwned>(clientdata:client::Client,tokenremove:&str) -> Result<T,E> {
    auth::remove_token(clientdata.clone(),tokenremove)
}
