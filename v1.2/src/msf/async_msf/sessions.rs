//! To handle the sessions in Metasploit RPC
#![allow(non_camel_case_types)]
use crate::client::Client;
use crate::error::MsfError;
use std::collections::HashMap;
use crate::response as res;
use crate::msf::sessions;

/// To list all sessions
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,sessions};
/// use metasploit::response::sessions as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::list=sessions::list(client.clone()).await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn list(client:Client) -> Result<res::sessions::list,MsfError> {
    sessions::list(client.clone())
}
/// To stop a session
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,sessions};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,sessions::stop(client.clone(),"1").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn stop(client:Client,sessionidstr:&str) -> Result<bool,MsfError> {
    sessions::stop(client.clone(),sessionidstr)
}
/// To read and write in shell
pub struct shell;
impl shell {
    /// To read a shell
    /// 
    /// ## Example
    /// ```
    /// use metasploit::client::Client;
    /// use metasploit::msf::{auth,sessions};
    /// use metasploit::response::sessions as resp;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(),Error> {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let response:resp::shell_read=sessions::shell::read(client.clone(),"1",None).await.unwrap();
    ///     println!("{:?}",response);
    ///     auth::logout(client.clone()).await.unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub async fn read(client:Client,sessionidstr:&str,readpointer:Option<i32>) -> Result<res::sessions::shell_read,MsfError> {
        sessions::shell::read(client.clone,sessionidstr,readpointer)
    }
    /// To write in a shell
    ///
    /// ## Example
    /// ```
    /// use metasploit::client::Client;
    /// use metasploit::msf::{auth,sessions};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(),Error> {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let response:String=sessions::shell::write(client.clone(),"1","help\n").await.unwrap();
    ///     println!("{}",response);
    ///     auth::logout(client.clone()).await.unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub async fn write(client:Client,sessionidstr:&str,datastr:&str) -> Result<String,MsfError> {
        sessions::shell::write(client.clone(),sessionidstr,datastr)
    }
}
/// To handle the meterpreter session.
pub struct meterpreter {
    /// Session ID of the meterpreter shell
    pub sessionid:String,
    /// Get the Client struct
    pub client:Client,
}
impl meterpreter {
    /// To create a new instance and store in a variable
    ///
    /// ## Example
    /// ```
    /// use metasploit::client::Client;
    /// use metasploit::msf::{auth,sessions};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(),Error> {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let meterpreter=sessions::meterpreter::new(client.clone(),"1");
    ///     let response= // Replace the variable with following examples
    ///     println!("{:?}",response);
    ///     auth::logout(client.clone()).await.unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client:Client,sessionidstr:&str) -> Self {
        meterpreter {
            sessionid:sessionidstr.to_string(),
            client:client,
        }
    }
    /// To write in a meterpreter shell
    ///
    /// It is recommended to add "\n" at the end of the command to execute
    /// ## Example
    /// ```
    /// let response=meterpreter.write("help\n").await.unwrap();
    /// ```
    pub async fn write(&self,datastr:&str) -> Result<bool,MsfError> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),self.sessionidstr);
        mtpr.write(datastr)
    }
    /// To read a meterpreter shell
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.read().await.unwrap();
    /// ```
    pub async fn read(&self) -> Result<String,MsfError> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),self.sessionidstr);
        mtpr.read()
    }
    /// To run a single command
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.run_single("help\n").await.unwrap();
    /// ```
    pub async fn run_single(&self,commandstr:&str) -> Result<bool,MsfError> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),self.sessionidstr);
        mtpr.run_single(commandstr)
    }
    /// To execute a given script
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.script("name.rb").await.unwrap();
    /// ```
    pub async fn script(&self,scriptnamestr:&str) -> Result<bool,MsfError> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),self.sessionidstr);
        mtpr.script(scriptnamestr)
    }
    /// To detach the meterpreter session
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.detach_session().await.unwrap();
    /// ```
    pub async fn detach_session(&self) -> Result<bool,MsfError> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),self.sessionidstr);
        mtpr.detach_session()
    }
    /// To kill a meterpreter shell
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.kill_session().await.unwrap();
    /// ```
    pub async fn kill_session(&self) -> Result<bool,MsfError> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),self.sessionidstr);
        mtpr.kill_session()
    }
    /// To get the list of all possible commands with a specific keyword
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.tabs("hel").await.unwrap();
    /// ```
    pub async fn tabs(&self,inputlinestr:&str) -> Result<Vec<String>,MsfError> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),self.sessionidstr);
        mtpr.tabs(inputlinestr)
    }
    /// To list all the compactible modules with the session
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.compactible_modules().await.unwrap();
    /// ```
    pub async fn compactible_modules(&self) -> Result<Vec<String>,MsfError> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),self.sessionidstr);
        mtpr.compactible_modules()
    }
}
/// To make a new meterpreter session from an existing shell
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,sessions};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,sessions::shell_upgrade(client.clone(),"1","127.0.0.1",8008).await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
///     Ok(())
/// }
/// ```
pub async fn shell_upgrade(client:Client,sessionidstr:&str,connecthoststr:&str,connectport:i32) -> Result<bool,MsfError> {
    sessions::shell_upgrade(client.clone(),sessionidstr,connecthoststr,connectport)
}
/// Ring module
pub struct ring {
    /// Get the Client
    client:Client,
    /// SessionID
    sessionid:String,
}
impl ring {
    /// To create a instance and store in the variable
    ///
    /// ## Example
    /// ```
    /// use metasploit::client::Client;
    /// use metasploit::msf::{auth,sessions};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(),Error> {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let ring=sessions::ring::new(client.clone(),"1");
    ///     let response= // Replace this variable with following examples
    ///     println!("{:?}",response);
    ///     auth::logout(client.clone()).await.unwrap();
    /// }
    /// ```
    pub fn new(client:Client,sessionid:&str) -> Self {
        ring {
            client:client,
            sessionid:sessionid.to_string(),
        }
    }
    /// To clear the ring buffer
    ///
    /// ## Example
    /// ```
    /// let response=ring.clear().await.unwrap();
    /// ```
    pub async fn clear(&self) -> Result<bool,MsfError> {
        let rng=sessions::ring::new(self.client.clone(),self.sessionid);
        rng.clear()
    }
    /// To get the last issued ReadPointer
    /// 
    /// ## Example
    /// ```
    /// let response=ring.last().await.unwrap();
    /// ```
    pub async fn last(&self) -> Result<i32,MsfError> {
        let rng=sessions::ring::new(self.client.clone(),self.sessionid);
        rng.last()
    }
    /// To write data into an active shell session
    ///
    /// ## Example
    /// ```
    /// let response=ring.put("data").await.unwrap(); 
    /// ```
    pub async fn put(&self,datastr:&str) -> Result<i32,MsfError> {
        let rng=sessions::ring::new(self.client.clone(),self.sessionid);
        rng.put(datastr)
    }
}
