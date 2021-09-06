#![allow(non_camel_case_types)]
#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
use connect::connect;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::{Error as derror,from_read}};
use crate::client::Client;
use error::MsfError;
use std::collections::HashMap;
use structs::{request as req,response as res};

pub fn list(client:Client) -> Result<res::sessions::list,MsfError> {
    let mut test:Result<res::sessions::list,MsfError>=Ok(HashMap::new());
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
            let de_ret:Result<res::sessions::list,derror>=Deserialize::deserialize(&mut de);
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
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
    match con {
        Ok(_) => {
            let de_ret:Result<res::sessions::stop,derror>=Deserialize::deserialize(&mut de);
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
pub struct shell;
impl shell {
    pub fn read(client:Client,sessionid:String,readpointer:Option<i32>) -> Result<res::sessions::shell_read,MsfError> {
        let mut test:Result<res::sessions::shell_read,MsfError>=Ok(res::sessions::shell_read {
            seq:1,
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
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
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
    pub fn write(client:Client,sessionid:String,data:String) -> Result<String,MsfError> {
        let mut test:Result<String,MsfError>=Ok(String::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::sessions::shell_write("session.shell_write".to_string(),client.token.unwrap(),sessionid,data);
        byte.serialize(&mut se).unwrap();
        let con = connect(client.url,body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::shell_write,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(ref val) = de_ret {
                    test=Ok(val.write_count.clone());
                };
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
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
    fn serialize(&self,body:&mut Vec<u8>,method:&str,param:Option<String>) {
        let mut se=Serializer::new(body);
        match param {
            Some(val) => {
                let byte=req::sessions::meterpreter_with_two(method.to_string(),self.client.token.as_ref().unwrap().to_string(),self.sessionid.clone(),val);
                byte.serialize(&mut se).unwrap();
            },
            None => {
                let byte=req::sessions::meterpreter_with_one(method.to_string(),self.client.token.as_ref().unwrap().to_string(),self.sessionid.clone());
                byte.serialize(&mut se).unwrap();
            },
        }
    }
    pub fn write(&self,data:String) -> Result<bool,MsfError> {
        let mut test:Result<bool,MsfError>=Ok(true);
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_write",Some(data));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::meterpreter_write,derror>=Deserialize::deserialize(&mut de);
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
    pub fn read(&self) -> Result<String,MsfError> {
        let mut test:Result<String,MsfError>=Ok(String::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_read",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::meterpreter_read,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(ref val) = de_ret {
                    test=Ok(val.data.clone());
                };
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        
        test
    }
    pub fn run_single(&self,command:String) -> Result<bool,MsfError> {
        let mut test:Result<bool,MsfError>=Ok(true);
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_run_single",Some(command));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::meterpreter_run_single,derror>=Deserialize::deserialize(&mut de);
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
    pub fn script(&self,scriptname:String) -> Result<bool,MsfError> {
        let mut test:Result<bool,MsfError>=Ok(true);
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_script",Some(scriptname));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::meterpreter_script,derror>=Deserialize::deserialize(&mut de);
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
    pub fn session_detach(&self) -> Result<bool,MsfError> {
        let mut test:Result<bool,MsfError>=Ok(true);
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_session_detach",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::meterpreter_session_detach,derror>=Deserialize::deserialize(&mut de);
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
    pub fn session_kill(&self) -> Result<bool,MsfError> {
        let mut test:Result<bool,MsfError>=Ok(true);
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_session_kill",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::meterpreter_session_kill,derror>=Deserialize::deserialize(&mut de);
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
    pub fn tabs(&self,inputline:String) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_tabs",Some(inputline));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::meterpreter_tabs,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(ref val) = de_ret {
                    test=Ok(val.tabs.clone());
                };  
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
    pub fn compactible_modules(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.compatible_modules",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::compactible_modules,derror>=Deserialize::deserialize(&mut de);
                if let Ok(ref val) = de_ret {
                    test=Ok(val.modules.clone());
                };
                if let Err(_) = de_ret {
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                    test=Err(de_ret);
                };
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            }
        }
        test
    }
}
pub fn shell_upgrade(client:Client,sessionid:String,connecthost:String,connectport:i32) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(true);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::sessions::shell_upgrade("session.shell_upgrade".to_string(),client.token.as_ref().unwrap().to_string(),sessionid,connecthost,connectport);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::sessions::shell_upgrade,derror>=Deserialize::deserialize(&mut de);
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
pub struct ring {
    client:Client,
    sessionid:String,
}
impl ring {
    pub fn new(client:Client,sessionid:String) -> Self {
        ring {
            client:client,
            sessionid:sessionid,
        }
    }
    fn serialize(&self,body:&mut Vec<u8>,method:&str,arg:Option<String>) {
        let mut se=Serializer::new(body);
        match arg {
            Some(val) => {
                let byte=req::sessions::ring_with_arg(method.to_string(),self.client.token.as_ref().unwrap().to_string(),self.sessionid.clone(),val);
                byte.serialize(&mut se).unwrap();
            },
            None => {
                let byte_new=req::sessions::ring_without_arg(method.to_string(),self.client.token.as_ref().unwrap().to_string(),self.sessionid.clone());
                byte_new.serialize(&mut se).unwrap();
            },
        }
    }
    pub fn clear(&self) -> Result<bool,MsfError> {
        let mut test:Result<bool,MsfError>=Ok(true);
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.ring_clear",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
             Ok(_) => {
                let de_ret:Result<res::sessions::ring_clear,derror>=Deserialize::deserialize(&mut de);
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
             }
        }
        test
    }
    pub fn last(&self) -> Result<i32,MsfError> {
        let mut test:Result<i32,MsfError>=Ok(1);
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.ring_last",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::ring_last,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(ref val) = de_ret {
                    test=Ok(val.seq);
                };
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
    pub fn put(&self,data:String) -> Result<i32,MsfError> {
        let mut test:Result<i32,MsfError>=Ok(1);
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.ring_put",Some(data));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::sessions::ring_put,derror>=Deserialize::deserialize(&mut de);
                if let Ok(ref val) = de_ret {
                    test=Ok(val.write_count.parse::<i32>().unwrap());
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
}
