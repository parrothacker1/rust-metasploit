//! A module to handle msfconsole.
#![allow(non_snake_case)]
use crate::error::Error as E;
use crate::client::Client;
#[path="../blocking/console.rs"] mod console;
use serde::de::DeserializeOwned as DOwned;

/// To Create a new console shell
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{console,auth};
/// use metasploit::response::console as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error>{
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let newconsole:resp::create=console::create(client.clone()).await.unwrap();
///     println!("{:?}",newconsole);
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn create<T:DOwned>(client:Client) -> Result<T,E> {
    console::create(client.clone())
}
/// To kill the existing specified console
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,console};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,console::destroy(client.clone(),"1").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn destroy<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    console::destroy(client.clone(),consoleID)
}
/// To get the list of all consoles
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,console};
/// use metasploit::response::console as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::list=console::list(client.clone()).await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn list<T:DOwned>(client:Client) -> Result<T,E> {
	console::list(client.clone())
}
/// To write a command into the shell.
///
/// It is recommended to add "\n" at the end of command.Or it may not execute
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,console};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(1,console::write(client.clone(),"1","help\n").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn write<T:DOwned>(client:Client,consoleID:&str,command:&str) -> Result<T,E> {
    console::write(client.clone(),consoleID,command)
}
/// To read the console
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,console};
/// use metasploit::response::console as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::read=console::read(client.clone(),"1").await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn read<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    console::read(client.clone(),consoleID)
}
/// To detach the session
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,console};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,console::detach_session(client.clone(),"1").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn detach_session<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    console::detach_session(client.clone(),consoleID)
}
/// To kill the session
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,console};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,console::kill_session(client.clone(),"1").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn kill_session<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    console::kill_session(client.clone(),consoleID)
}
/// To list all the possible commands which starts with a specific keyword
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,console};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     println!("{:?}",console::tabs(client.clone(),"1","hel").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn tabs<T:DOwned>(client:Client,consoleID:&str,inputlinestr:&str) -> Result<T,E> {
    console::tabs(client.clone(),consoleID,inputlinestr)
}
