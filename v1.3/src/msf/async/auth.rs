//! Module whcih contain all functions for authentication
use crate::client;
#[path="../blocking/auth.rs"] mod auth;
use crate::error::Error as E;
use serde::de::DeserializeOwned as DOwned;

/// To logout from the RPC Server
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::async::auth;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,auth::logout(client.clone()).await.unwrap());
///     Ok(())
/// }
/// ```
pub async fn logout<T:DOwned>(clientdata:client::Client) -> Result<T,E> {
    auth::logout(clientdata.clone())
}
/// To add a new token to RPC Server
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::async::auth;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,auth::add_token(client.clone(),"newtoken").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn add_token<T:DOwned>(clientdata:client::Client,newtokenstr:&str) -> Result<T,E> {
    auth::add_token(clientdata.clone(),newtokenstr)
}
/// To Generate the token
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::async::auth;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     auth::add_token(client.clone(),"newtoken").await.unwrap();
///     assert_eq!("newtoken",auth::generate_token(client.clone()).await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn generate_token<T:DOwned>(clientdata:client::Client) -> Result<T,E> {
    auth::generate_token(clientdata.clone())
}
/// To list all the tokens registered with RPC Server
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::async::auth;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     auth::add_token(client.clone(),"newtoken").await.unwrap();
///     println!("{:?}",auth::list_token(client.clone()).await.unwrap()); // ["newtoken","<rpctoken>"]
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn list_token<T:DOwned>(clientdata:client::Client) -> Result<T,E> {
    auth::list_token(clientdata.clone())
}
/// To remove a token from the RPC Server
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::async::auth;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     auth::add_token(client.clone(),"newtoken").await.unwrap();
///     assert_eq!(true,auth::remove_token(client.clone(),"newtoken").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn remove_token<T:DOwned>(clientdata:client::Client,tokenremove:&str) -> Result<T,E> {
    auth::remove_token(clientdata.clone(),tokenremove)
}
