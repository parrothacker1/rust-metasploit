#![allow(non_snake_case)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use crate::error::{MsfError,Error as E};
use connect::connect;
use structs::request as req;
use crate::client::Client;
use serde::{Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};

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
