#![allow(non_camel_case_types)]
#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
use crate::client::Client;
use connect::connect;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::Error as derror};
use error::MsfError;
use structs::{request as req,response as res};

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
    fn deserialize(&self,buf:Vec<u8>) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut de=Deserializer::new(buf.as_slice());
        let de_ret:Result<res::modules::list,derror>=Deserialize::deserialize(&mut de);
        if let Ok(ref val) = de_ret {
            test=Ok(val.modules.clone());
        };
        if let Err(_) = de_ret {
            let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
            test=Err(de_ret);
        };
        test
    }
    pub fn exploits(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.exploits",&mut body);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                test=self.deserialize(new_buf);
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
    pub fn auxiliary(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.auxiliary",&mut body);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                test=self.deserialize(new_buf);
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
    pub fn post(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.post",&mut body);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                test=self.deserialize(new_buf);
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
    pub fn payloads(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.payloads",&mut body);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                test=self.deserialize(new_buf);
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        
        test
    }
    pub fn encoders(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.encoders",&mut body);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                test=self.deserialize(new_buf);
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
    pub fn nops(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        self.serialize("module.nops",&mut body);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        match con {
            Ok(_) => {
                test=self.deserialize(new_buf);
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
}
pub fn info(client:Client,moduletype:String,modulename:String) -> Result<res::modules::info,MsfError> {
    let mut test:Result<res::modules::info,MsfError>=Ok(res::modules::info {
        name:String::new(),
        description:String::new(),
        license:String::new(),
        filepath:String::new(),
        version:0,
        rank:0,
        authors:Vec::new(),
        references:Vec::new(),
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::modules::info("module.info".to_string(),client.token.unwrap(),moduletype,modulename);
    byte.serialize(&mut se);
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::modules::info,derror>=Deserialize::deserialize(&mut de);
            if let Ok(ref val) = de_ret {
                test=Ok(val.clone());
            };
            if let Err(_) = de_ret {
                let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                test=Err(de_ret)
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
impl compactible {
    pub fn new(modulename:String,client:Client) -> Self {
        compactible {
            name:modulename,
            client:client,
        }
    }
    pub fn payload(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::modules::compactible("module.compactible_payloads".to_string(),self.client.token.as_ref().unwrap().to_string(),self.name.clone());
        byte.serialize(&mut se);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::modules::compactible_payloads,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(ref val) = de_ret {
                    test=Ok(val.payloads.clone());
                };
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
    pub fn target_payload(&self,targetindx:i32) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::modules::compactible_tp("module.target_compactible_payloads".to_string(),self.client.token.as_ref().unwrap().to_string(),self.name.clone(),targetindx);
        byte.serialize(&mut se);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::modules::compactible_payloads,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(ref val) = de_ret {
                    test=Ok(val.payloads.clone());
                };
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
    pub fn sessions(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::modules::compactible("module.compactible_sessions".to_string(),self.client.token.as_ref().unwrap().to_string(),self.name.clone());
        byte.serialize(&mut se);
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::modules::compactible_sessions,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(ref val) = de_ret {
                    test=Ok(val.sessions.clone());
                };
            },
            Err(_) => {
                panic!("Connection closed unexpectedly");
            },
        }
        test
    }
}
pub fn option(client:Client,moduletype:String,modulename:String) -> Result<HashMap<String,res::modules::options>,MsfError> {
    let mut test:Result<HashMap<String,res::modules::options>,MsfError>=Ok(HashMap::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::modules::options("module.options".to_string(),client.token.as_ref().unwrap().to_string(),moduletype.clone(),modulename.clone());
    byte.serialize(&mut serializer);
    let con=connect(client.url.clone(),body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<HashMap<String,res::modules::options>,derror>=Deserialize::deserialize(&mut de);
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
pub fn encoder(client:Client,data:String,encodermodule:String,options:HashMap<String,String>) -> Result<String,MsfError> {
    let mut test:Result<String,MsfError>=Ok(String::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::modules::encoder("module.encode".to_string(),client.token.unwrap(),data,encodermodule,options);
    byte.serialize(&mut se);
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::modules::encode,derror>=Deserialize::deserialize(&mut de);
            if let Ok(ref val) = de_ret {
                test=Ok(val.encoded.clone());
            };
            if let Err(_) = de_ret {
                let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                test=Err(de_ret);
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
pub fn execute(client:Client,moduletype:String,modulename:String,options:HashMap<String,String>) -> Result<String,MsfError> {
    let mut test:Result<String,MsfError>=Ok(String::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::modules::execute("module.execute".to_string(),client.token.unwrap(),moduletype.clone(),modulename,options);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
                let de_ret_p:Result<res::modules::execute_payloads,derror>=Deserialize::deserialize(&mut de);
            if moduletype.clone()=="payload".to_string() {
                if let Err(_) = de_ret_p {
                    let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(val) = de_ret_p {
                    test=Ok(val.payload);
                };
            } else {
                let de_ret:Result<res::modules::execute_non_payloads,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(val) = de_ret {
                    test=Ok(val.job_id.to_string());
                };
            }
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
