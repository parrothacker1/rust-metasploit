//! A module which is used to handle msfcore
#![allow(non_snake_case)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use crate::error::{MsfError,Error as E};
use crate::client::Client;
use connect::connect;
use serde::{Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};
use structs::request as req;

/// To add a new module by path
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// use metasploit::response::core as resp;
/// 
/// fn main() {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::addmodpath=core::add_module(client.clone(),"path").unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn add_module<T:DOwned>(client:Client,pathstr:&str) -> Result<T,E> {
    let path:String=pathstr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::core::addmodpath("core.add_module_path".to_string(),client.token.unwrap(),path);
    byte.serialize(&mut serializer).unwrap();
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
/// To get the status of modules loaded
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// use metasploit::response::core as resp;
///
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::modulestat=core::module_status(client.clone()).unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn module_status<T:DOwned>(client:Client) -> Result<T,E> { 
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::modulestat("core.module_stats".to_string(),client.token.unwrap());
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
/// To reload all the modules
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// use metasploit::response::core as resp;
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::reloadmod=core::reload_modules(client.clone()).unwrap();
///     println!("{:?}",response)
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn reload_module<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::reloadmod("core.reload_modules".to_string(),client.token.unwrap());
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
/// To save in the core
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::save(client.clone()).unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn save<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::save("core.save".to_string(),client.token.unwrap());
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
/// To set setg with key value pair
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::setg(client.clone(),"name","value").unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn setg<T:DOwned>(client:Client,namestr:&str,valuestr:&str) -> Result<T,E> {
    let name:String=namestr.to_string();
    let value:String=valuestr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::setg("core.setg".to_string(),client.token.unwrap(),name,value);
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
/// To remove setg with key name
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::unsetg(client.clone(),"name").unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn unsetg<T:DOwned>(client:Client,namestr:&str) -> Result<T,E> {
    let name:String=namestr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::unsetg("core.unsetg".to_string(),client.token.unwrap(),name);
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
/// To list all the threads
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// use metasploit::response::core as resp;
/// use std::collections::HashMap;
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:HashMap<i32,resp::threadlist>=core::list_thread(client.clone()).unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn list_thread<T:DOwned>(client:Client) -> Result<T,E> {
	let mut body=Vec::new();
	let mut buf=vec![];
	let mut se=Serializer::new(&mut body);
	let byte=req::core::threadlist("core.thread_list".to_string(),client.token.unwrap());
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
/// To kill a thread
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::kill_thread(client.clone(),1).unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn kill_thread<T:DOwned>(client:Client,threadID:i32) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::threadkill("core.thread_kill".to_string(),client.token.unwrap(),threadID);
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
/// To get the version
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,core::version(client.clone()).unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn version<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::version("core.version".to_string(),client.token.unwrap());
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
/// To stop the core
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,core};
/// 
/// fn main() {
///     let client=Client::new("127.0.0.1",55552,,"msf","password",true);
///     assert_eq!(true,core::stop(client.clone()).unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn stop<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::stop("core.stop".to_string(),client.token.unwrap());
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
