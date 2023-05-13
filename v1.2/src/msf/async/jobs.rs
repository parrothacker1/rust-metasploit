use crate::client::Client;
use crate::error::Error as E;
#[path="../blocking/jobs.rs"] mod jobs;
use serde::de::DeserializeOwned as DOwned;

pub async fn list<T:DOwned>(client:Client) -> Result<T,E> {
    jobs::list(client.clone())
}
pub async fn info<T:DOwned>(client:Client,jobidstr:&str) -> Result<T,E> {
    jobs::info(client.clone(),jobidstr)
}
pub async fn stop<T:DOwned>(client:Client,jobidstr:&str) -> Result<T,E> {
    jobs::stop(client.clone(),jobidstr)
}
