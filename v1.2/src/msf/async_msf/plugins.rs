//! A module to handle plugins in Metasploit RPC
#[path="../../structs/mod.rs"] mod structs;
#[path="../../error.rs"] mod error;
#[path="../../connect.rs"] mod connect;
use crate::client::Client;
use connect::connect;
use std::collections::HashMap;
use crate::error::MsfError;
use crate::msf::plugins;

/// To load a plugin
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,plugins};
/// use std::collections::HashMap;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let option=HashMap::new();
///     option.insert("key".to_string(),"value".to_string());
///     assert_eq!(true,plugins::load(client.clone(),"pluginname",option).await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn load(client:Client,pluginnamestr:&str,options:HashMap<String,String>) -> Result<bool,MsfError> {
    plugins::load(client.clone(),pluginnamestr,options)
}
/// To unload a plugin
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,plugins};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,plugins::unload(client.clone(),"pluginname").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn unload(client:Client,pluginnamestr:&str) -> Result<bool,MsfError> {
    plugins::unload(client.clone(),pluginnamestr)
}
/// To list all the loaded plugins
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,plugins};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:Vec<String>=plugins::list(client.clone()).await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn list(client:Client) -> Result<Vec<String>,MsfError> {
    plugins::list(client.clone())
}
