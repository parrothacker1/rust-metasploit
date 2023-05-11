//! A module to handle plugins in Metasploit RPC
#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
use connect::connect_async;
use std::collections::HashMap;
use error::MsfError;
use structs::{request as req,response as res};
use crate::client::Client;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::{Error as derror,from_read}};

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
    let pluginname:String=pluginnamestr.to_string();
    let mut test:Result<bool,MsfError>=Ok(true);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::plugins::load("plugin.load".to_string(),client.token.unwrap(),pluginname,options);
    byte.serialize(&mut se).unwrap();
    let con=connect_async(client.url,body,&mut buf).await;
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::plugins::load,derror>=Deserialize::deserialize(&mut de);
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            };
            if let Ok(ref val) = de_ret {
                if val.result=="success".to_string() {
                    test=Ok(true);
                } else {
                    test=Ok(false);
                }
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
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
    let pluginname:String=pluginnamestr.to_string();
    let mut test:Result<bool,MsfError>=Ok(true);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::plugins::unload("plugin.unload".to_string(),client.token.unwrap(),pluginname);
    byte.serialize(&mut se).unwrap();
    let con=connect_async(client.url,body,&mut buf).await;
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::plugins::unload,derror>=Deserialize::deserialize(&mut de);
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            };
            if let Ok(ref val) = de_ret {
                if val.result=="success".to_string() {
                    test=Ok(true);
                } else {
                    test=Ok(false);
                }
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
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
    let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::plugins::loaded("plugin.loaded".to_string(),client.token.unwrap());
    byte.serialize(&mut se).unwrap();
    let con=connect_async(client.url,body,&mut buf).await;
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::plugins::loaded,derror>=Deserialize::deserialize(&mut de);
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            };
            if let Ok(ref val) = de_ret {
                test=Ok(val.plugins.clone())
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}

