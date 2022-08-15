//! A module to handle plugins in Metasploit RPC
use crate::client::Client;
use std::collections::HashMap;
use crate::error::Error as E;
#[path="../blocking/plugins.rs"] mod plugins;
use serde::de::DeserializeOwned as DOwned;

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
pub async fn load<T:DOwned>(client:Client,pluginnamestr:&str,options:HashMap<String,String>) -> Result<T,E> {
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
pub async fn unload<T:DOwned>(client:Client,pluginnamestr:&str) -> Result<T,E> {
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
pub async fn list<T:DOwned>(client:Client) -> Result<T,E> {
    plugins::list(client.clone())
}
