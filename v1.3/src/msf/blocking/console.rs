//! A module to handle msfconsole.
#![allow(non_snake_case)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use crate::error::{MsfError,Error as E};
use connect::connect;
use structs::request as req;
use crate::client::Client;
use serde::{Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};

/// To Create a new console shell
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{console,auth};
/// use metasploit::response::console as resp;
///
/// fn main() {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let newconsole:resp::create=console::create(client.clone()).unwrap();
///     println!("{:?}",newconsole);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn create<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::create("console.create".to_string(),client.token.as_ref().unwrap().to_string());
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
/// To kill the existing specified console
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,console};
/// 
/// fn main() {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,console::destroy(client.clone(),"1").unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn destroy<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    let consoleid:String=consoleID.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::destroy("console.destroy".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer).unwrap();
    let new_buf=buf.clone();
    let con=connect(client.url,body,&mut buf);
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
/// To get the list of all consoles
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,console};
/// use metasploit::response::console as resp;
///
/// fn main() {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::list=console::list(client.clone()).unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn list<T:DOwned>(client:Client) -> Result<T,E> {
	let mut body=Vec::new();
	let mut buf=vec![];
	let mut serializer=Serializer::new(&mut body);
	let byte=req::console::list("console.list".to_string(),client.token.unwrap());
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
/// To write a command into the shell.
///
/// It is recommended to add "\n" at the end of command.Or it may not execute
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,console};
/// 
/// fn main() {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(1,console::write(client.clone(),"1","help\n").unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn write<T:DOwned>(client:Client,consoleID:&str,command:&str) -> Result<T,E> {
    let data:String=command.to_string();
    let consoleid:String=consoleID.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::write("console.write".to_string(),client.token.unwrap(),consoleid,data);
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
/// To read the console
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,console};
/// use metasploit::response::console as resp;
/// 
/// fn main() {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::read=console::read(client.clone(),"1").unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn read<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    let consoleid:String=consoleID.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::read("console.read".to_string(),client.token.unwrap(),consoleid);
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
/// To detach the session
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,console};
/// 
/// fn main() {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,console::detach_session(client.clone(),"1").unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn detach_session<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    let consoleid:String=consoleID.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::session_detach("console.session_detach".to_string(),client.token.unwrap(),consoleid);
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
/// To kill the session
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,console};
/// 
/// fn main() {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,console::kill_session(client.clone(),"1").unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn kill_session<T:DOwned>(client:Client,consoleID:&str) -> Result<T,E> {
    let consoleid:String=consoleID.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::session_kill("console.session_kill".to_string(),client.token.unwrap(),consoleid);
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
/// To list all the possible commands which starts with a specific keyword
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::blocking::{auth,console};
/// 
/// fn main() {
///     let client:Client=Client::new("127.0.0.1",55552,"msf","password",true);
///     println!("{:?}",console::tabs(client.clone(),"1","hel").unwrap());
///     auth::logout(client.clone()).unwrap();
/// }
/// ```
pub fn tabs<T:DOwned>(client:Client,consoleID:&str,inputlinestr:&str) -> Result<T,E> {
    let consoleid:String=consoleID.to_string();
    let inputline:String=inputlinestr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::tabs("console.tabs".to_string(),client.token.unwrap(),consoleid,inputline);
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
