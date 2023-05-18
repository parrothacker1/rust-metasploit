#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use crate::client::Client;
use connect::connect;
use std::collections::HashMap;
use serde::{Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};
use crate::error::{MsfError,Error as E};
use structs::request as req;

pub struct compactible {
    pub name:String,
    pub client:Client,
}
pub struct list {
    pub client:Client,
}
impl list {
    pub fn new(client:Client) -> Self {
        list {
            client:client,
        }
    }
    fn serialize(&self,method:&str,body:&mut Vec<u8>) {
        let mut se=Serializer::new(body);
        let byte=req::modules::list(method.to_string(),self.client.token.as_ref().unwrap().to_string());
        byte.serialize(&mut se).unwrap();
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
            },
        }
    }
    pub fn exploits<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.exploits",&mut body);
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
    pub fn auxiliary<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.auxiliary",&mut body);
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
    pub fn post<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.post",&mut body);
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
    pub fn payloads<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.payloads",&mut body);
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
    pub fn encoders<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.encoders",&mut body);
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
    pub fn nops<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.nops",&mut body);
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
pub fn info<T:DOwned>(client:Client,moduletypestr:&str,modulenamestr:&str) -> Result<T,E> {
    let moduletype:String=moduletypestr.to_string();
    let modulename:String=modulenamestr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::modules::info("module.info".to_string(),client.token.unwrap(),moduletype,modulename);
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
impl compactible {
    pub fn new(modulename:String,client:Client) -> Self {
        compactible {
            name:modulename,
            client:client,
        }
    }
    pub fn payload<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::modules::compactible("module.compatible_payloads".to_string(),self.client.token.as_ref().unwrap().to_string(),self.name.clone());
        byte.serialize(&mut se).unwrap();
        let con=connect(self.client.url.clone(),body,&mut buf);
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
    pub fn target_payloads<T:DOwned>(&self,targetindx:i32) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::modules::compactible_tp("module.target_compatible_payloads".to_string(),self.client.token.as_ref().unwrap().to_string(),self.name.clone(),targetindx);
        byte.serialize(&mut se).unwrap();
        let con=connect(self.client.url.clone(),body,&mut buf);
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
    pub fn sessions<T:DOwned>(&self) -> Result<T,E> {
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::modules::compactible("module.compatible_sessions".to_string(),self.client.token.as_ref().unwrap().to_string(),self.name.clone());
        byte.serialize(&mut se).unwrap();
        let con=connect(self.client.url.clone(),body,&mut buf);
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
pub fn option<T:DOwned>(client:Client,moduletypestr:&str,modulenamestr:&str) -> Result<T,E> {
    let moduletype:String=moduletypestr.to_string();
    let modulename:String=modulenamestr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::modules::options("module.options".to_string(),client.token.as_ref().unwrap().to_string(),moduletype.clone(),modulename.clone());
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url.clone(),body,&mut buf);
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
pub fn encoder<T:DOwned>(client:Client,datastr:&str,encodermodulestr:&str,options:HashMap<String,String>) -> Result<T,E> {
    let data:String=datastr.to_string();
    let encodermodule:String=encodermodulestr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::modules::encoder("module.encode".to_string(),client.token.unwrap(),data,encodermodule,options);
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
pub fn execute<T:DOwned>(client:Client,moduletypestr:&str,modulenamestr:&str,options:HashMap<String,String>) -> Result<T,E> {
    let moduletype:String=moduletypestr.to_string();
    let modulename:String=modulenamestr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::modules::execute("module.execute".to_string(),client.token.unwrap(),moduletype.clone(),modulename,options);
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
