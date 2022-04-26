//! A module to handle msfconsole.
#![allow(non_snake_case)]
use crate::error::MsfError;
use crate::response as res;
use crate::client::Client;
use crate::msf::console;

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
pub async fn create(client:Client) -> Result<res::console::create,MsfError> {
    let test:Result<res::console::create,MsfError>=console::create(client.clone());
    test
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
pub async fn destroy(client:Client,consoleID:&str) -> Result<bool,MsfError> {
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
pub async fn list(client:Client) -> Result<res::console::list,MsfError> {
	let test:Result<res::console::list,MsfError>=console::list(client.clone());
    test
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
pub async fn write(client:Client,consoleID:&str,command:&str) -> Result<i32,MsfError> {
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
pub async fn read(client:Client,consoleID:&str) -> Result<res::console::read,MsfError> {
    let test:Result<res::console::read,MsfError>=console::read(client.clone(),consoleID);
    test
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
pub async fn detach_session(client:Client,consoleID:&str) -> Result<bool,MsfError> {
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
pub async fn kill_session(client:Client,consoleID:&str) -> Result<bool,MsfError> {
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
pub async fn tabs(client:Client,consoleID:&str,inputlinestr:&str) -> Result<Vec<String>,MsfError> {
    console::tabs(client.clone(),consoleID,inputlinestr)
}
