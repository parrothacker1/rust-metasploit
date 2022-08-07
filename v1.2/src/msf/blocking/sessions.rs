//! To handle the sessions in Metasploit RPC
#![allow(non_camel_case_types)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use connect::connect;
use serde::{Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};
use crate::client::Client;
use crate::error::{MsfError,Error as E};
use structs::request as req;

/// To list all sessions
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,sessions};
/// use metasploit::response::sessions as resp;
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::list=sessions::list(client.clone()).unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
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
/// To stop a session
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,sessions};
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,sessions::stop(client.clone(),"1").unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
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
/// To read and write in shell
pub struct shell;
impl shell {
    /// To read a shell
    /// 
    /// ## Example
    /// ```
    /// use metasploit::client::Client;
    /// use metasploit::msf::blocking::{auth,sessions};
    /// use metasploit::response::sessions as resp;
    ///
    /// fn main() {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let response:resp::shell_read=sessions::shell::read(client.clone(),"1",None).unwrap();
    ///     println!("{:?}",response);
    ///     auth::logout(client.clone()).unwrap();
    ///     Ok(())
    /// }
    /// ```
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
    /// To write in a shell
    ///
    /// ## Example
    /// ```
    /// use metasploit::client::Client;
    /// use metasploit::msf::blcoking::{auth,sessions};
    /// 
    /// fn main() {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let response:String=sessions::shell::write(client.clone(),"1","help\n").unwrap();
    ///     println!("{}",response);
    ///     auth::logout(client.clone()).unwrap();
    /// }
    /// ```
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
    /// use metasploit::client::Client;
    /// use metasploit::msf::blocking::{auth,sessions};
    /// 
    /// fn main() {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let meterpreter=sessions::meterpreter::new(client.clone(),"1");
    ///     let response= // Replace the variable with following examples
    ///     println!("{:?}",response);
    ///     auth::logout(client.clone()).unwrap();
    /// }
    /// ```
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
    /// To write in a meterpreter shell
    ///
    /// It is recommended to add "\n" at the end of the command to execute
    /// ## Example
    /// ```
    /// let response=meterpreter.write("help\n").unwrap();
    /// ```
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
    /// To read a meterpreter shell
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.read().unwrap();
    /// ```
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
    /// To run a single command
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.run_single("help\n").unwrap();
    /// ```
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
    /// To execute a given script
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.script("name.rb").unwrap();
    /// ```
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
    /// To detach the meterpreter session
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.detach_session().unwrap();
    /// ```
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
    /// To kill a meterpreter shell
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.kill_session().unwrap();
    /// ```
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
    /// To get the list of all possible commands with a specific keyword
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.tabs("hel").unwrap();
    /// ```
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
    /// To list all the compactible modules with the session
    ///
    /// ## Example
    /// ```
    /// let response=meterpreter.compactible_modules().unwrap();
    /// ```
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
/// To make a new meterpreter session from an existing shell
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,sessions};
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,sessions::shell_upgrade(client.clone(),"1","127.0.0.1",8008).unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
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
    /// use metasploit::client::Client;
    /// use metasploit::msf::blocking::{auth,sessions};
    /// 
    /// fn main() {
    ///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
    ///     let ring=sessions::ring::new(client.clone(),"1");
    ///     let response= // Replace this variable with following examples
    ///     println!("{:?}",response);
    ///     auth::logout(client.clone()).unwrap();
    /// }
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
    /// To clear the ring buffer
    ///
    /// ## Example
    /// ```
    /// let response=ring.clear().unwrap();
    /// ```
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
    /// To get the last issued ReadPointer
    /// 
    /// ## Example
    /// ```
    /// let response=ring.last().unwrap();
    /// ```
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
    /// To write data into an active shell session
    ///
    /// ## Example
    /// ```
    /// let response=ring.put("data").unwrap(); 
    /// ```
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
