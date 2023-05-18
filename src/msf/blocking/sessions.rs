#![allow(non_camel_case_types)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use connect::connect;
use serde::{Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};
use crate::client::Client;
use crate::error::{MsfError,Error as E};
use structs::request as req;

pub fn list<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::sessions::list("session.list".to_string(),client.token.unwrap());
    byte.serialize(&mut se).unwrap();
    let con = connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    match con {
        Ok(_) => {
            let ret:Result<T,derror>=from_read(new_buf.as_slice());
            match ret {
                Ok(val) => {
                    Ok(val)
                },
                Err(_) => {
                    let ret2:Result<MsfError,derror>=from_read(new_buf.as_slice());
                    match ret2 {
                        Ok(val) => {
                            Err(E::MsfError(val))
                        },
                        Err(e) => {
                            Err(E::DError(e))
                        },
                    }
                }
            }
        },
        Err(e) => {
            Err(E::ConnectionError(e))
        },
    }
}
pub fn stop<T:DOwned>(client:Client,sessionidstr:&str) -> Result<T,E> {
    let sessionid:String=sessionidstr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::sessions::stop("session.stop".to_string(),client.token.unwrap(),sessionid);
    byte.serialize(&mut se).unwrap();
    let con = connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    match con {
        Ok(_) => {
            let ret:Result<T,derror>=from_read(new_buf.as_slice());
            match ret {
                Ok(val) => {
                    Ok(val)
                },
                Err(_) => {
                    let ret2:Result<MsfError,derror>=from_read(new_buf.as_slice());
                    match ret2 {
                        Ok(val) => {
                            Err(E::MsfError(val))
                        },
                        Err(e) => {
                            Err(E::DError(e))
                        },
                    }
                }
            }
        },
        Err(e) => {
            Err(E::ConnectionError(e))
        },
    }
}
pub struct shell;
impl shell {
    pub fn read<T:DOwned>(client:Client,sessionidstr:&str,readpointer:Option<i32>) -> Result<T,E> {
        let sessionid:String=sessionidstr.to_string();
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
        match con {
            Ok(_) => {
                let ret:Result<T,derror>=from_read(new_buf.as_slice());
                match ret {
                    Ok(val) => {
                        Ok(val)
                    },
                    Err(_) => {
                        let ret2:Result<MsfError,derror>=from_read(new_buf.as_slice());
                        match ret2 {
                            Ok(val) => {
                                Err(E::MsfError(val))
                            },
                            Err(e) => {
                                Err(E::DError(e))
                            },
                        }
                    }
                }
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn write<T:DOwned>(client:Client,sessionidstr:&str,datastr:&str) -> Result<T,E> {
        let sessionid:String=sessionidstr.to_string();
        let data:String=datastr.to_string();
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::sessions::shell_write("session.shell_write".to_string(),client.token.unwrap(),sessionid,data);
        byte.serialize(&mut se).unwrap();
        let con = connect(client.url,body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                let ret:Result<T,derror>=from_read(new_buf.as_slice());
                match ret {
                    Ok(val) => {
                        Ok(val)
                    },
                    Err(_) => {
                        let ret2:Result<MsfError,derror>=from_read(new_buf.as_slice());
                        match ret2 {
                            Ok(val) => {
                                Err(E::MsfError(val))
                            },
                            Err(e) => {
                                Err(E::DError(e))
                            },
                        }
                    }
                }
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
}
pub struct meterpreter {
    pub sessionid:String,
    pub client:Client,
}
impl meterpreter {
    pub fn new(client:Client,sessionidstr:&str) -> Self {
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
    fn deserialize<T:DOwned>(&self,new_buf:Vec<u8>) -> Result<T,E> {
        let ret:Result<T,derror>=from_read(new_buf.as_slice());
        match ret {
            Ok(val) => {
                Ok(val)
            },
            Err(_) => {
                let ret2:Result<MsfError,derror>=from_read(new_buf.as_slice());
                match ret2 {
                    Ok(val) => {
                        Err(E::MsfError(val))
                    },
                    Err(e) => {
                        Err(E::DError(e))
                    },
                }
            }
        }
    }
    pub fn write<T:DOwned>(&self,datastr:&str) -> Result<T,E> {
        let data:String=datastr.to_string();
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_write",Some(data));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn read<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_read",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn run_single<T:DOwned>(&self,commandstr:&str) -> Result<T,E> {
        let command:String=commandstr.to_string();
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_run_single",Some(command));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn script<T:DOwned>(&self,scriptnamestr:&str) -> Result<T,E> {
        let scriptname:String=scriptnamestr.to_string();
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_script",Some(scriptname));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn detach_session<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_session_detach",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn kill_session<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_session_kill",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn tabs<T:DOwned>(&self,inputlinestr:&str) -> Result<T,E> {
        let inputline=inputlinestr.to_string();
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.meterpreter_tabs",Some(inputline));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn compactible_modules<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.compatible_modules",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
}
pub fn shell_upgrade<T:DOwned>(client:Client,sessionidstr:&str,connecthoststr:&str,connectport:i32) -> Result<T,E> {
    let sessionid:String=sessionidstr.to_string();
    let connecthost:String=connecthoststr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::sessions::shell_upgrade("session.shell_upgrade".to_string(),client.token.as_ref().unwrap().to_string(),sessionid,connecthost,connectport);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    match con {
        Ok(_) => {
            let ret:Result<T,derror>=from_read(new_buf.as_slice());
            match ret {
                Ok(val) => {
                    Ok(val)
                },
                Err(_) => {
                    let ret2:Result<MsfError,derror>=from_read(new_buf.as_slice());
                    match ret2 {
                        Ok(val) => {
                            Err(E::MsfError(val))
                        },
                        Err(e) => {
                            Err(E::DError(e))
                        },
                    }
                }
            }
        },
        Err(e) => {
            Err(E::ConnectionError(e))
        },
    }
}
pub struct ring {
    client:Client,
    sessionid:String,
}
impl ring {
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
    fn deserialize<T:DOwned>(&self,new_buf:Vec<u8>) -> Result<T,E> {
        let ret:Result<T,derror>=from_read(new_buf.as_slice());
        match ret {
            Ok(val) => {
                Ok(val)
            },
            Err(_) => {
                let ret2:Result<MsfError,derror>=from_read(new_buf.as_slice());
                match ret2 {
                    Ok(val) => {
                        Err(E::MsfError(val))
                    },
                    Err(e) => {
                        Err(E::DError(e))
                    },
                }
            }
        }
    }
    pub fn clear<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.ring_clear",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn last<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.ring_last",None);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
    pub fn put<T:DOwned>(&self,datastr:&str) -> Result<T,E> {
        let data:String=datastr.to_string();
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize(&mut body,"session.ring_put",Some(data));
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                self.deserialize(new_buf)
            },
            Err(e) => {
                Err(E::ConnectionError(e))
            },
        }
    }
}
