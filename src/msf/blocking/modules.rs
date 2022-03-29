//! A module to handle all the modules in Metasploit RPC
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../error.rs"] mod error;
#[path="../../connect.rs"] mod connect;
use crate::{value::Value,client::Client};
use connect::connect;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::{Error as derror,from_read}};
use error::MsfError;
use structs::{request as req,response as res};

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
    /// use metasploit::msf::blocking::{auth,modules};
    ///
    /// fn main() -> Result<(),Error> {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let list=modules::list::new(client.clone());
    ///     let resp= // Replace the variable with the following
    ///     println!("{:?}",resp);
    ///     auth::logout(client.clone()).unwrap();
    /// }
    /// ```
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
            let de_ret:MsfError=from_read(buf.as_slice()).unwrap();
            test=Err(de_ret);
        };
        test
    }
    /// To list all exploits
    ///
    /// ## Example
    /// ```
    /// let resp=list.exploits().unwrap();
    /// ```
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
    /// To list all auxiliaries
    ///
    /// ## Example
    /// ```
    /// let resp=list.auxiliary().unwrap();
    /// ```
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
    /// To list all posts
    ///
    /// ## Example
    /// ```
    /// let resp=list.posts().unwrap();
    /// ```
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
    /// To list all payloads
    ///
    /// ## Example
    /// ```
    /// let resp=list.payloads().unwrap();
    /// ```
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
    /// To list all encoders
    ///
    /// ## Example
    /// ```
    /// let resp=list.encoders().unwrap();
    /// ```
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
    /// To list all nops
    /// 
    /// ## Example
    /// ```
    /// let resp=list.nops().unwrap();
    /// ```
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
/// To get information about the module
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,modules};
/// use metasploit::response::modules as resp;
///
/// fn main() {
///     let client=Client::new("127.0.0.1",55553,"msf","password",true);
///     let response:resp::info=module::info(client.clone(),"moduletype","modulename").unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn info(client:Client,moduletypestr:&str,modulenamestr:&str) -> Result<res::modules::info,MsfError> {
    let moduletype:String=moduletypestr.to_string();
    let modulename:String=modulenamestr.to_string();
    let mut test:Result<res::modules::info,MsfError>=Err(MsfError {
        error:true,
        error_class:String::new(),
        error_string:String::new(),
        error_message:String::new(),
        error_backtrace:Vec::new(),
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::modules::info("module.info".to_string(),client.token.unwrap(),moduletype,modulename);
    byte.serialize(&mut se).unwrap();
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
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret)
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
/// To get the list of compactible payloads and sessions
impl compactible {
    /// To create a new instance and store the value in a variable
    ///
    /// ## Example
    /// ```
    /// use meetasploit::client::Client;
    /// use metasploit::msf::blocking::{auth,modules};
    /// 
    /// fn main() {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let compactible=modules::compactible::new("modulename",client.clone());
    ///     let response= // Replace the variable with following example ones
    ///     println!("{:?}",response);
    ///     auth::logout(client.clone()).unwrap();
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
    /// let response=compactible.payloads().unwrap();
    /// ```
    pub fn payload(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::modules::compactible("module.compatible_payloads".to_string(),self.client.token.as_ref().unwrap().to_string(),self.name.clone());
        byte.serialize(&mut se).unwrap();
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::modules::compactible_payloads,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
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
    /// To get a list of compactible payloads for a specific target
    ///
    /// ## Example
    /// ```
    /// let response=compactible.target_payloads(1).unwrap();
    /// ```
    pub fn target_payloads(&self,targetindx:i32) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::modules::compactible_tp("module.target_compatible_payloads".to_string(),self.client.token.as_ref().unwrap().to_string(),self.name.clone(),targetindx);
        byte.serialize(&mut se).unwrap();
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::modules::compactible_payloads,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
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
    /// To get a list of sessions
    ///
    /// ## Example
    /// ```
    /// let response=compactible.sessions().unwrap();
    /// ```
    pub fn sessions(&self) -> Result<Vec<String>,MsfError> {
        let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut se=Serializer::new(&mut body);
        let byte=req::modules::compactible("module.compatible_sessions".to_string(),self.client.token.as_ref().unwrap().to_string(),self.name.clone());
        byte.serialize(&mut se).unwrap();
        let con=connect(self.client.url.clone(),body,&mut buf);
        let new_buf=buf.clone();
        let mut de=Deserializer::new(new_buf.as_slice());
        match con {
            Ok(_) => {
                let de_ret:Result<res::modules::compactible_sessions,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
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
/// To get the options of a module
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,modules};
/// use metasploit::response::modules as resp;
/// use std::collections::HashMap;
///
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:HashMap<String,resp::options>=modules::option(client.clone(),"moduletype","modulename").unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn option(client:Client,moduletypestr:&str,modulenamestr:&str) -> Result<HashMap<String,res::modules::options>,MsfError> {
    let moduletype:String=moduletypestr.to_string();
    let modulename:String=modulenamestr.to_string();
    let mut test:Result<HashMap<String,res::modules::options>,MsfError>=Ok(HashMap::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::modules::options("module.options".to_string(),client.token.as_ref().unwrap().to_string(),moduletype.clone(),modulename.clone());
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url.clone(),body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<HashMap<String,res::modules::options>,derror>=Deserialize::deserialize(&mut de);
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
/// To encode a module
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,modules};
/// use std::collections::HashMap;
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let option=HashMap::new();
///     option.insert("key".to_string(),"value".to_string());
///     let response:String=module::encoder(client.clone(),"data","encodermodule",option).unwrap();
///     println!("{}",response);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn encoder(client:Client,datastr:&str,encodermodulestr:&str,options:HashMap<String,String>) -> Result<String,MsfError> {
    let data:String=datastr.to_string();
let encodermodule:String=encodermodulestr.to_string();
    let mut test:Result<String,MsfError>=Ok(String::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::modules::encoder("module.encode".to_string(),client.token.unwrap(),data,encodermodule,options);
    byte.serialize(&mut se).unwrap();
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
/// To execute a module
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,modules};
/// use metasploit::value::Value;
/// use std::collections::HashMap;
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let option=HashMap::new();
///     option.insert("key".to_string(),"value".to_string());
///     let response:Value=modules::execute(client.clone(),"moduletype","modulename",option).unwrap();
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn execute(client:Client,moduletypestr:&str,modulenamestr:&str,options:HashMap<String,String>) -> Result<Value,MsfError> {
    let moduletype:String=moduletypestr.to_string();
    let modulename:String=modulenamestr.to_string();
    let mut test:Result<Value,MsfError>=Ok(Value::from(true));
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
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(val) = de_ret_p {
                    test=Ok(val.payload);
                };
            } else {
                let de_ret:Result<res::modules::execute_non_payloads,derror>=Deserialize::deserialize(&mut de);
                if let Err(_) = de_ret {
                    let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                    test=Err(de_ret);
                };
                if let Ok(val) = de_ret {
                    test=Ok(Value::from(val.job_id));
                };
            }
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
