#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use crate::error::{MsfError,Error as E};
use crate::client;
use structs::request as req;
use serde::{Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};

pub fn logout<T:DOwned>(clientdata:client::Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::logout("auth.logout".to_string(),clientdata.token.as_ref().unwrap().to_string(),clientdata.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(clientdata.url,body,&mut buf);
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
pub fn add_token<T:DOwned>(clientdata:client::Client,newtokenstr:&str) -> Result<T,E> {
    let new_tok:String=newtokenstr.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::tokenadd("auth.token_add".to_string(),clientdata.token.as_ref().unwrap().to_string(),new_tok);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(clientdata.url,body,&mut buf);
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
pub fn generate_token<T:DOwned>(clientdata:client::Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::tokengen("auth.token_generate".to_string(),clientdata.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    let mut buf=vec![];
    let con=connect::connect(clientdata.url,body,&mut buf);
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
pub fn list_token<T:DOwned>(clientdata:client::Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::tokenlist("auth.token_list".to_string(),clientdata.token.unwrap());
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(clientdata.url,body,&mut buf);
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
pub fn remove_token<T:DOwned>(clientdata:client::Client,tokenremove:&str) -> Result<T,E> {
    let token_rem:String=tokenremove.to_string();
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::auth::tokenrem("auth.token_remove".to_string(),clientdata.token.unwrap(),token_rem);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(clientdata.url,body,&mut buf);
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
