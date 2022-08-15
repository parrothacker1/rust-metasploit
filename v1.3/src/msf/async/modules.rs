//! A module to handle all the modules in Metasploit RPC
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
use crate::client::Client;
use std::collections::HashMap;
use crate::error::Error as E;
use serde::de::DeserializeOwned as DOwned;
#[path="../blocking/modules.rs"] mod modules;

/// To list the compactible payloads and sessions
pub struct compactible {
    /// Name of the module
    pub name:String,
    /// Get the Client struct
    pub client:Client,
}
/// To list exploits,auxiliary,posts,payloads,nops,encoders
pub struct list {
    /// Get the client struct
    pub client:Client,
}
impl list {
    /// To create a new variable with list value
    ///
    /// ## Example
    /// ```
    /// use metasploit::client::Client;
    /// use metasploit::msf::{auth,modules};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(),Error> {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let list=modules::list::new(client.clone());
    ///     let resp= // Replace the variable with the following
    ///     println!("{:?}",resp);
    ///     auth::logout(client.clone()).await.unwrap();
    /// }
    /// ```
    pub fn new(client:Client) -> Self {
        list {
            client:client,
        }
    }
    /// To list all exploits
    ///
    /// ## Example
    /// ```
    /// let resp=list.exploits().await.unwrap();
    /// ```
    pub async fn exploits<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.exploits()
    }
    /// To list all auxiliaries
    ///
    /// ## Example
    /// ```
    /// let resp=list.auxiliary().await.unwrap();
    /// ```
    pub async fn auxiliary<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.auxiliary()
    }
    /// To list all posts
    ///
    /// ## Example
    /// ```
    /// let resp=list.posts().await.unwrap();
    /// ```
    pub async fn post<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.post()
    }
    /// To list all payloads
    ///
    /// ## Example
    /// ```
    /// let resp=list.payloads().await.unwrap();
    /// ```
    pub async fn payloads<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.payloads()
    }
    /// To list all encoders
    ///
    /// ## Example
    /// ```
    /// let resp=list.encoders().await.unwrap();
    /// ```
    pub async fn encoders<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.encoders()
    }
    /// To list all nops
    /// 
    /// ## Example
    /// ```
    /// let resp=list.nops().await.unwrap();
    /// ```
    pub async fn nops<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::list::new(self.client.clone());
        list.nops()
    }
}
/// To get information about the module
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,modules};
/// use metasploit::response::modules as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55553,"msf","password",true);
///     let response:resp::info=module::info(client.clone(),"moduletype","modulename").await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn info<T:DOwned>(client:Client,moduletypestr:&str,modulenamestr:&str) -> Result<T,E> {
    modules::info(client.clone(),moduletypestr,modulenamestr)
}
/// To get the list of compactible payloads and sessions
impl compactible {
    /// To create a new instance and store the value in a variable
    ///
    /// ## Example
    /// ```
    /// use meetasploit::client::Client;
    /// use metasploit::msf::{auth,modules};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(),Error> {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let compactible=modules::compactible::new("modulename",client.clone());
    ///     let response= // Replace the variable with following example ones
    ///     println!("{:?}",response);
    ///     auth::logout(client.clone()).await.unwrap();
    /// }
    /// ```
    pub fn new(modulename:String,client:Client) -> Self {
        compactible {
            name:modulename,
            client:client,
        }
    }
    /// To get a list of compactible payloads
    ///
    /// ## Example
    /// ```
    /// let response=compactible.payloads().await.unwrap();
    /// ```
    pub async fn payload<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::compactible::new(self.name.clone(),self.client.clone());
        list.payload()
    }
    /// To get a list of compactible payloads for a specific target
    ///
    /// ## Example
    /// ```
    /// let response=compactible.target_payloads(1).await.unwrap();
    /// ```
    pub async fn target_payloads<T:DOwned>(&self,targetindx:i32) -> Result<T,E> {
        let list=modules::compactible::new(self.name.clone(),self.client.clone());
        list.target_payloads(targetindx)
    }
    /// To get a list of sessions
    ///
    /// ## Example
    /// ```
    /// let response=compactible.sessions().await.unwrap();
    /// ```
    pub async fn sessions<T:DOwned>(&self) -> Result<T,E> {
        let list=modules::compactible::new(self.name.clone(),self.client.clone());
        list.sessions()
    }
}
/// To get the options of a module
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,modules};
/// use metasploit::response::modules as resp;
/// use std::collections::HashMap;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:HashMap<String,resp::options>=modules::option(client.clone(),"moduletype","modulename").await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn option<T:DOwned>(client:Client,moduletypestr:&str,modulenamestr:&str) -> Result<T,E> {
    modules::option(client.clone(),moduletypestr,modulenamestr)
}
/// To encode a module
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,modules};
/// use std::collections::HashMap;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let option=HashMap::new();
///     option.insert("key".to_string(),"value".to_string());
///     let response:String=module::encoder(client.clone(),"data","encodermodule",option).await.unwrap();
///     println!("{}",response);
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn encoder<T:DOwned>(client:Client,datastr:&str,encodermodulestr:&str,options:HashMap<String,String>) -> Result<T,E> {
    modules::encoder(client.clone(),datastr,encodermodulestr,options)
}
/// To execute a module
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,modules};
/// use metasploit::value::Value;
/// use std::collections::HashMap;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let option=HashMap::new();
///     option.insert("key".to_string(),"value".to_string());
///     let response:Value=modules::execute(client.clone(),"moduletype","modulename",option).await.unwrap();
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn execute<T:DOwned>(client:Client,moduletypestr:&str,modulenamestr:&str,options:HashMap<String,String>) -> Result<T,E> {
    modules::execute(client.clone(),moduletypestr,modulenamestr,options)
}
