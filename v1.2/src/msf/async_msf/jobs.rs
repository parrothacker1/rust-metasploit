//! A module to handle the jobs in Metasploit
use crate::client::Client;
use crate::error::MsfError;
use std::collections::HashMap;
use crate::response as res;
use crate::msf::jobs;

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
pub async fn list(client:Client) -> Result<HashMap<String,String>,MsfError> {
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
pub async fn info(client:Client,jobidstr:&str) -> Result<res::jobs::info,MsfError> {
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
pub async fn stop(client:Client,jobidstr:&str) -> Result<bool,MsfError> {
    jobs::stop(client.clone(),jobidstr)
}
