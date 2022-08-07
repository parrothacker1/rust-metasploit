//! A module to handle the jobs in Metasploit
use crate::client::Client;
use crate::error::Error as E;
#[path="../blocking/jobs.rs"] mod jobs;
use serde::de::DeserializeOwned as DOwned;

/// To list all the currently running jobs
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,job};
/// use std::collections::HashMap;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:HashMap<String,String>=jobs::list(client.clone()).await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn list<T:DOwned>(client:Client) -> Result<T,E> {
    jobs::list(client.clone())
}
/// To get information about the specified job
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,jobs};
/// use metasploit::response::jobs as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::info=jobs::info(client.clone(),"1").await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn info<T:DOwned>(client:Client,jobidstr:&str) -> Result<T,E> {
    jobs::info(client.clone(),jobidstr)
}
/// To stop a specified job
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,jobs};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Cluent::mew("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,jobs::stop(client.clone(),"1").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn stop<T:DOwned>(client:Client,jobidstr:&str) -> Result<T,E> {
    jobs::stop(client.clone(),jobidstr)
}
