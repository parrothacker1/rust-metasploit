//! A module to manage postgres and credentials.
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use structs::{request as req,response as res};
use serde::{Serialize,Deserialize};
use connect::connect;
use rmp_serde::{Serializer,Deserializer,{decode::Error as derror,from_read}};
use crate::{error::MsfError,client::Client};
use std::collections::HashMap;

/// To add a new workspace
///
/// ## Example
/// ```
/// db::add_workspace(client.clone(),"new_workspace").unwrap(); //true
/// ```
pub fn add_workspace(client:Client,wname:&str) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(true);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::db::add_workspace("db.add_workspace".to_string(),client.token.unwrap(),wname.to_string());
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::db::add_workspace,derror>=Deserialize::deserialize(&mut de);
            if let Ok(ref val) = de_ret {
                if val.result=="success".to_string() {
                    test=Ok(true);
                } else {
                    test=Ok(false);
                }
            };
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}

/// To analyse the host
///
/// ## Example
/// ```
///
/// ```
//pub fn analyse_hosts(client:Client,wname:String,host:String,)

/// To get the current workspace
///
/// ## Example
/// ```
///
/// ```
pub fn current_workspace(client:Client) -> Result<HashMap<i32,String>,MsfError> {
    let mut test:Result<HashMap<i32,String>,MsfError>=Ok(HashMap::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let se=Serializer::new(&mut body);
    let byte=req::
}
pub fn hosts(client:Client,xopts:Option<HashMap<String,String>>) -> Result<res::db::hosts,MsfError> {
    let mut test:Result<res::db::hosts,MsfError>=Err(MsfError {
        error:true,
        error_class:String::new(),
        error_string:String::new(),
        error_message:String::new(),
        error_backtrace:Vec::new(),
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let opts:HashMap<String,String>;
    if xopts==None {
        opts=HashMap::new();
    } else {
        opts=xopts.unwrap();
    }
    let byte=req::db::hosts("db.hosts".to_string(),opts);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::db::hosts,derror>=Deserialize::deserialize(&mut de);
            if let Ok(ref val) = de_ret {
                test=Ok(val.clone());
            }
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            }
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        }
    }
    test
}
