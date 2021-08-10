#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
use connect::connect;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::Error as derror};
use crate::client::Client;
use error::MsfError;
use std::collections::HashMap;
use structs::{request as req,response as res};

pub fn list(client:Client) -> Result<HashMap<String,res::sessions::list>,MsfError> {
    let mut test:Result<HashMap<String,res::sessions::list>,MsfError>=Ok(HashMap::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::sessions::list("session.list".to_string(),client.token.unwrap());
    byte.serialize(&mut se).unwrap();
    let con = connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<HashMap<String,res::sessions::list>,derror>=Deserialize::deserialize(&mut de);
            if let Err(_) = de_ret {
                let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                test=Err(de_ret);
            };
            if let Ok(ref val) = de_ret {
                test=Ok(val.clone());
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
pub fn stop(client:Client,sessionid:String) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(true);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::sessions::stop("session.stop".to_string(),client.token.unwrap(),sessionid);
    byte.serialize(&mut se).unwrap();
    let con = connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    let con=connect(client.url,body,&mut buf);
    match con {
        Ok(_) => {
            let de_ret:Result<res::sessions::stop,derror>=Deserialize::deserialize(&mut de);
            if let Err(_) = de_ret {
                let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
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
pub enum shell {
    read(),
    write(),
}
impl shell {
    pub fn read(client:Client,sessionid:String,readpointer:Option<i32>) -> Result<res::sessions::shell_read,MsfError> {
        let mut test:Result<res::sessions::shell_read,MsfError>=Ok(res::sessions::shell_read {
            seq:String::new(),
            data:String::new(),
        });
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        match readpointer {
            Some(_) => {
                let byte=req::sessions::shell_read_with_pointer("session.shell_read".to_string(),client.token.unwrap(),sessionid,readpointer.unwrap());
                byte.serialize(&mut se).unwrap();
            },
            None => {
                let byte=req::sessions::shell_read("session.shell_read".to_string(),client.token.unwrap(),sessionid);
                byte.serialize(&mut se).unwrap();
            },
        }
        let con = connect(client.url,body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::shell_read,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(ref val) = de_ret {
                    test=Ok(val.clone());
                };
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
    pub fn write(client:Client,sessionid:String) -> Result<i32,MsfError> {
        let test:i32=1;
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::sessions::list("session.list".to_string(),client.token.unwrap());
        byte.serialize(&mut se).unwrap();
        let con = connect(client.url,body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        Ok(test)
    }
}
pub struct meterpreter {
    pub sessionid:String,
    pub client:Client,
}
impl meterpreter {
    pub fn new(client:Client,sessionid:String) -> Self {
        meterpreter {
            sessionid:sessionid,
            client:client,
        }
    }
    pub fn write(&self,data:String) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn read(&self) -> Result<String,MsfError> {
        let test:String=String::new();
        Ok(test)
    }
    pub fn run_single(&self,command:String) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn script(&self,scriptname:String) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn session_detach(&self) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn session_kill(&self) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    pub fn tabs(&self,inputline:String) -> Result<Vec<String>,MsfError> {
        let test:Vec<String>=Vec::new();
        Ok(test)
    }
}
pub fn compactible_modules(client:Client,sessionid:String) -> Result<Vec<String>,MsfError> {
    let test:Vec<String>=Vec::new();
    Ok(test)
}
pub fn shell_upgrade(client:Client,sessionid:String,connecthost:String,connectport:i32) -> Result<bool,MsfError> {
    let test:bool=true;
    Ok(test)
}
pub trait ring {
    fn new(sessionid:String) -> Self;
    fn clear(&self) -> Result<bool,MsfError> {
        let test:bool=true;
        Ok(test)
    }
    fn last(&self) -> Result<i32,MsfError> {
        let test:i32=1;
        Ok(test)
    }
    fn put(&self,data:String) -> Result<i32,MsfError> {
        let test:i32=1;
        Ok(test)
    }
}
