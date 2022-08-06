//! A module which is used to handle msfcore
#![allow(non_snake_case)]
use crate::error::Error as E;
use crate::client::Client;
use crate::msf::core;
use serde::de::DeserializeOwned as DOwned;

/// To add a new module by path
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use metasploit::response::core as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::addmodpath=core::add_module(client.clone(),"path").await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn add_module<T:DOwned>(client:Client,pathstr:&str) -> Result<T,E> {
    core::add_module(client.clone(),pathstr)
}
/// To get the status of modules loaded
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use metasploit::response::core as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::modulestat=core::module_status(client.clone()).await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn module_status<T:DOwned>(client:Client) -> Result<T,E> {
    core::module_status(client.clone())
}
/// To reload all the modules
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use metasploit::response::core as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::reloadmod=core::reload_modules(client.clone()).await.unwrap();
///     println!("{:?}",response)
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn reload_module<T:DOwned>(client:Client) -> Result<T,E> {
    core::reload_module(client.clone())
}
/// To save in the core
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::save(client.clone()).await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn save<T:DOwned>(client:Client) -> Result<T,E> {
    core::save(client.clone())
}
/// To set setg with key value pair
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::setg(client.clone(),"name","value").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn setg<T:DOwned>(client:Client,namestr:&str,valuestr:&str) -> Result<T,E> {
    core::setg(client.clone(),namestr,valuestr)
}
/// To remove setg with key name
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::unsetg(client.clone(),"name").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn unsetg<T:DOwned>(client:Client,namestr:&str) -> Result<T,E> {
    core::unsetg(client.clone(),namestr)
}
/// To list all the threads
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use metasploit::response::core as resp;
/// use std::collections::HashMap;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:HashMap<i32,resp::threadlist>=core::list_thread(client.clone()).await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn list_thread<T:DOwned>(client:Client) -> Result<T,E> {
    core::list_thread(client.clone())
}
/// To kill a thread
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::kill_thread(client.clone(),1).await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn kill_thread<T:DOwned>(client:Client,threadID:i32) -> Result<T,E> {
    core::kill_thread(client.clone(),threadID)
}
/// To get the version
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::version(client.clone()).await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn version<T:DOwned>(client:Client) -> Result<T,E> {
    core::version(client.clone())
}
/// To stop the core
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,core};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,,"msf","password",true);
///     assert_eq!(true,core::stop(client.clone()).await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn stop<T:DOwned>(client:Client) -> Result<T,E> {
    core::stop(client.clone())
}
