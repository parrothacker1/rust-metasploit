//! To handle the sessions in Metasploit RPC
#![allow(non_camel_case_types)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../error.rs"] mod error;
#[path="../../connect.rs"] mod connect;
use connect::connect;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::{Error as derror,from_read}};
use crate::client::Client;
use error::MsfError;
use std::collections::HashMap;
use structs::{request as req,response as res};

/// To list all sessions
///
/// ## Example
/// ```
/// sessions::list(client.clone()).await.unwrap(); // response::sessions::list {}
/// ```
pub async fn list(client:Client) -> Result<res::sessions::list,MsfError> {
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
/// To stop a session
///
/// ## Example
/// ```
/// sessions::stop(client.clone(),"1").await.unwrap(); // true
/// ```
pub async fn stop(client:Client,sessionidstr:&str) -> Result<bool,MsfError> {
    let sessionid:String=sessionidstr.to_string();
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
/// To read and write in shell
pub struct shell;
impl shell {
    /// To read a shell
    /// 
    /// ## Example
    /// ```
    /// sessions::shell::read(client.clone(),"1",None).await.unwrap(); // response::sessions::shell_read {};
    /// ```
    pub async fn read(client:Client,sessionidstr:&str,readpointer:Option<i32>) -> Result<res::sessions::shell_read,MsfError> {
        let sessionid:String=sessionidstr.to_string();
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
    /// To write in a shell
    ///
    /// ## Example
    /// ```
    /// sessions::shell::write(client.clone(),"1","help\n").await.unwrap(); //String
    /// ```
    pub async fn write(client:Client,sessionidstr:&str,datastr:&str) -> Result<String,MsfError> {
        let sessionid:String=sessionidstr.to_string();
        let data:String=datastr.to_string();
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
    /// let meterpreter=sessions::meterpreter::new(client.clone(),"1");
    /// ```
    pub async fn new(client:Client,sessionidstr:&str) -> Self {
        meterpreter {
            sessionid:sessionidstr.to_string(),
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
    /// To write in a meterpreter shell
    ///
    /// It is recommended to add "\n" at the end of the command to execute
    /// ## Example
    /// ```
    /// meterpreter.write("help\n").unwrap(); // true
    /// ```
    pub fn write(&self,datastr:&str) -> Result<bool,MsfError> {
        let data:String=datastr.to_string();
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
    /// To read a meterpreter shell
    ///
    /// ## Example
    /// ```
    /// meterpreter.read().unwrap(); //String
    /// ```
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
    /// To run a single command
    ///
    /// ## Example
    /// ```
    /// meterpreter.run_single("help\n").unwrap(); //true
    /// ```
    pub fn run_single(&self,commandstr:&str) -> Result<bool,MsfError> {
        let command:String=commandstr.to_string();
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
    /// To execute a given script
    ///
    /// ## Example
    /// ```
    /// meterpreter.script("name.rb").unwrap(); // true
    /// ```
    pub fn script(&self,scriptnamestr:&str) -> Result<bool,MsfError> {
        let scriptname:String=scriptnamestr.to_string();
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
    /// To detach the meterpreter session
    ///
    /// ## Example
    /// ```
    /// meterpreter.detach_session().unwrap(); // true
    /// ```
    pub fn detach_session(&self) -> Result<bool,MsfError> {
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
    /// To kill a meterpreter shell
    ///
    /// ## Example
    /// ```
    /// meterpreter.kill_session().unwrap(); // true
    /// ```
    pub fn kill_session(&self) -> Result<bool,MsfError> {
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
    /// To get the list of all possible commands with a specific keyword
    ///
    /// ## Example
    /// ```
    /// meterpreter.tabs("hel").unwrap(); // ["help"]
    /// ```
    pub fn tabs(&self,inputlinestr:&str) -> Result<Vec<String>,MsfError> {
        let inputline=inputlinestr.to_string();
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
    /// To list all the compactible modules with the session
    ///
    /// ## Example
    /// ```
    /// meterpreter.compactible_modules().unwrap(); //Vec<String>
    /// ```
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
/// To make a new meterpreter session from an existing shell
///
/// ## Example
/// ```
/// sessions::shell_upgrade(client.clone(),"1","127.0.0.1",8008).unwrap(); // true
/// ```
pub fn shell_upgrade(client:Client,sessionidstr:&str,connecthoststr:&str,connectport:i32) -> Result<bool,MsfError> {
    let sessionid:String=sessionidstr.to_string();
    let connecthost:String=connecthoststr.to_string();
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
    /// let ring=sessions::ring::new(client.clone(),"1");
    /// ```
    pub fn new(client:Client,sessionid:&str) -> Self {
        ring {
            client:client,
            sessionid:sessionid.to_string(),
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
    /// To clear the ring buffer
    ///
    /// ## Example
    /// ```
    /// ring.clear().unwrap(); // true
    /// ```
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
    /// To get the last issued ReadPointer
    /// 
    /// ## Example
    /// ```
    /// ring.last().unwrap(); // 3
    /// ```
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
    /// To write data into an active shell session
    ///
    /// ## Example
    /// ```
    /// ring.put("data").unwrap(); // 4 
    /// ```
    pub fn put(&self,datastr:&str) -> Result<i32,MsfError> {
        let data:String=datastr.to_string();
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
