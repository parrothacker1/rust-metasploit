use crate::client::Client;
use crate::error::Error as E;
#[path="../blocking/custom.rs"] mod custom;
use serde::{ser::Serialize,de::DeserializeOwned as DOwned};

pub async fn custom<T:DOwned,S:Serialize>(client:Client,byte:S) -> Result<T,E> {
    custom::custom(client,byte)
}
