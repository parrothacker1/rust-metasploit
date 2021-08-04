#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use connect::connect;
use error::conerr;
use std::collections::HashMap;
use serde::Serialize as se;
use rmp_serde::Serializer;
use serde_json::value::from_value;
use common::{ReturnValue as Return_Type,MsfError,sessionlist,sessionread};

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
#[derive(se)]
struct sessionstruct(String,String);
pub fn list(client:Client) -> Return_Type {
	let test;
	let mut body=Vec::new();
	let mut serializer=Serializer::new(&mut body);
	let byte=sessionstruct("session.list".to_string(),client.token.unwrap());
	byte.serialize(&mut serializer).unwrap();
	let con=connect(client.url,body);
	match con {
		Ok(val) => {
			let ret:Result<HashMap<String,sessionlist>,serde_json::Error>=from_value(val);
			match ret {
				Ok(val) => {
					test=Return_Type::SessionList(val);
				},
				Err(_e) => {
					let ret:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(ret);
				},
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
	test
}
#[derive(se)]
struct sessionstop(String,String,String);
pub fn stop(client:Client,sessionid:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=sessionstop("session.stop".to_string(),client.token.unwrap(),sessionid);
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("result") == None {
                let ret:MsfError = from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                if val.get("result").unwrap().as_str().unwrap() == "success" {
                    test=Return_Type::Bool(true);
                } else {
                    test=Return_Type::Bool(false);
                }
			}
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub enum shell {
    read(),
    write(),
}
#[derive(se)]
struct shellread(String,String,String);
#[derive(se)]
struct shellreadwithrp(String,String,Sting,String);
impl shell {
    pub fn read(client:Client,sessionid:String,readpointer:Option<String>) -> Return_Type {
        let test;
        let byte;
        let mut body=Vec::new();
        let mut serializer=Serializer::new(&mut body);
        if readpointer == None {
			byte=shellread("session.shell_read".to_string(),client.token.unwrap(),sessionid);
		} else {
			byte=shellreadwithrp("session.shell_read".to_string(),client.token.unwrap(),sessionid,readpointer.unwrap());
		}
        byte.serialize(&mut serializer).unwrap();
        let con=connect(client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<sessionread,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::SessionRead(val);
					},
					Err(_e) => {
						let ret:MsfError=from_value(val).unwrap();
						test=Return_Type::MsfErr(ret);
					},
				}
			},
			Err(_e) => {
				test=Return_Type::String(conerr::ConInterrupt.to_string());
			},
		}
        test
    }
    pub fn write(client:Client,sessionid:String,command:String) -> Return_Type {
        let test;
        let mut body=Vec::new();
        let mut serializer=Serializer::new(&mut body);
        let byte=shellreadwithrp("session.shell_write".to_string(),client.token.unwrap(),sessionid,command);
        byte.serialize(&mut serializer).unwrap();
        let con=connect(client.url,body);
        match con {
			Ok(val) => {
				if val.get("write_count")==None {
					let ret:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(ret);
				} else {
					test=Return_Type::Int(val.get("write_count").unwrap().as_i64().unwrap());
				}
			},
			Err(_e) => {
				test=Return_Type::String(conerr::ConInterrupt.to_string());
			},
		}
        test
    }
}
pub struct meterpreter {
    pub sessionid:String,
    pub client:Client,
}
#[derive(se)]
struct meterpreterwithoutarg(String,String,String);
#[derive(se)]
struct meterpreterwitharg(String,String,String,String);
impl meterpreter {
    pub fn new(client:Client,sessionid:String) -> Self {
        meterpreter {
            sessionid:sessionid,
            client:client,
        }
    }
    fn serialize_without_arg(&self,method:&str,body:&mut Vec<u8>) {
		let serialize=Serializer::new(&mut body);
		let byte=meterpreterwithoutarg(method.to_string(),self.client.token.unwrap(),self.sessionid);
		byte.serialize(&mut serializer).unwrap();
	}
	fn serialize_with_arg(&self,method:&str,arg:String,body:&mut Vec<u8>) {
		let serializer=Serializer::new(&mut body);
		let byte=meterpreterwitharg(method.to_string(),self.client.token.unwrap(),self.sessionid,arg);
		byte.serialize(&mut serializer).unwrap();
	}
    pub fn write(&self,data:String) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize_with_arg("session.meterpreter_write",data,&mut body);
        let con=connect(self.client.url,body);
        match con {
			Ok(val) => {
				if val.get("result").unwrap().as_str().unwrap()=="success" {
					test=Return_Type::Bool(true);
				} else {
					let ret:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(ret);
				}
			},
			Err(_e) => {
				test=Return_Type::String(conerr::ConInterrupt.to_string());
			},
		}
        test
    }
    pub fn read(&self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize_without_arg("session.meterpreter_read",&mut body);
        let con=connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<sessionread,serde_json::Error>=from_value(val);
				match ret {
					Ok(value) => {
						test=Return_Type::SessionRead(value);
					},
					Err(_e) => {
						let ret:MsfError=from_value(val).unwrap();
						test=Return_Type::MsfErr(ret);
					},
				}
			},
			Err(_e) => {
				test=Return_Type::String(conerr::ConInterrupt.to_string());
			},
		}
        test
    }
    pub fn run_single(&self,command:String) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize_with_arg("session.meterpreter_run_single",command,&mut body);
        let con=connect(self.client.url,body);
        match con {
			Ok(val) => {
				if val.get("result")!=None {
					if val.get("result").unwrap().as_str().unwrap()=="success" {
						test=Return_Type::Bool(true);
					} else {
						test=Return_Type::Bool(false);
					}
				} else {
					let ret:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(ret);
				}
			},
			Err(_e) => {
				test=Return_Type::String(conerr::ConInterrupt.to_string());
			},
		}
        test
    }
    pub fn script(&self,scriptname:String) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize_with_arg("session.meterpreter_script",scriptname,&mut body);
        let con=connect(self.client.url,body);
        match con {
			Ok(val) => {
				if val.get("result")==None {
					let ret:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(ret);
				} else {
					if val.get("result").unwrap().as_str().unwrap()=="success" {
						test=Return_Type::Bool(true);
					} else {
						test=Return_Type::Bool(false);
					}
				}
			},
			Err(_e) => {
				test=Return_Type::String(conerr::ConInterrupt.to_string());
			},
		}
        test
    }
    pub fn session_detach(&self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize_without_arg("session.meterpreter_session_detach",&mut body);
        let con=connect(self.client.url,body);
        match con {
			Ok(val) => {
				if val.get("result")==None {
					let ret:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(ret);
				} else {
					if val.get("result").unwrap().as_str().unwrap()=="success" {
						test=Return_Type::Bool(true);
					} else {
						test=Return_Type::Bool(false);
					}
				}
			},
			Err(_e) => {
				test=Return_Type::String(conerr::ConInterrupt.to_string());
			},
		}
        test
    }
    pub fn session_kill(&self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize_without_arg("session.meterpreter_session_kill",&mut body);
        let con=connect(self.client.url,body);
        match con {
			Ok(val) => {
				if val.get("result")==None {
					let ret:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(ret);
				} else {
					if val.get("result").unwrap().as_str().unwrap()=="success" {
						test=Return_Type::Bool(true);
					} else {
						test=Return_Type::Bool(false);
					}
				}
			},
			Err(_e) => {
				test=Return_Type::String(conerr::ConInterrupt.to_string());
			},
		}
        test
    }
    pub fn tabs(&self,inputline:String) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize_with_arg("session.meterpreter_tabs",inputline,&mut body);
        let con=connect(self.client.url,body);
        match con {
			Ok(val) => {
				if val.get("tabs")==None {
					let ret:MsfError=from_value(val).unwrap();
					test=Return_Type::MsfErr(ret);
				} else {
					let ret=val.get("tabs").unwrap().as_array().unwrap();
					test=Return_Type::ArrayStr(ret);
				}
			},
			Err(_e)=> {
				test=Return_Type::String(conerr::ConInterrupt.to_string());
			},
		}
        test
    }
}
pub fn compactible_modules(client:Client,sessionid:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=meterpreterwithoutarg("session.compactible_modules".to_string(),client.token.unwrap(),sessionid);
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url,body);
    match con {
		Ok(val) => {
			if val.get("modules") == None {
				let ret:MsfError=from_value(val).unwrap();
				test=Return_Type::MsfErr(ret);
			} else {
				let ret=val.get("modules").unwrap().as_array().unwrap();
				test=Return_Type::ArrayStr(ret);
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
#[derive(se)]
struct shellupgrade(String,String,String,String,i32);
pub fn shell_upgrade(client:Client,sessionid:String,connecthost:String,connectport:i32) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serailizer=Serializer::new(&mut body);
    let byte=shellupgrade("session.shell_upgrade".to_string(),client.token.unwrap(),sessionid,connecthost,connectport);
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("result")==None {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                if val.get("result").unwrap().as_str().unwrap()=="success" {
                    test=Return_Type::Bool(true);
                } else {
                    test=Return_Type::Bool(false);
                }
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub struct ring {
	sessionid:String,
	client:Client,
}
#[derive(se)]
struct shellring(String,String,String);
#[derive(se)]
struct shellringwitharg(String,String,String,String);

impl ring {
    pub fn new(sessionid:String,client:Client) -> Self {
		ring {
			sessionid:sessionid,
			client:client,
		}
	}
    fn serialize_without_arg(&self,method:&str,body:&mut Vec<u8>) {
        let serializer=Serializer::new(&mut body);
        let byte=shellring(method.to_string(),self.client.token.unwrap(),self.sessionid);
        byte.serialize(&mut serializer).unwrap();
    }
    fn serialize_with_arg(&self,method:&str,arg:String,body:&mut Vec<u8>) {
        let serializer=Serializer::new(&mut body);
        let byte=shellringwitharg(method.to_string(),self.client.token.unwrap(),self.sessionid,arg);
        byte.serialize(&mut serializer);
    }
    pub fn clear(&self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize_without_arg("session.ring_clear",&mut body);
        let con=connect(self.client.url,body);
        match con {
            Ok(val) => {
                if val.get("result")==None {
                    let ret:MsfError=from_value(val).unwrap();
                    test=Return_Type::MsfErr(ret);
                } else {
                    if val.get("result").unwrap().as_str().unwrap() == "success" {
                        test=Return_Type::Bool(true);
                    } else {
                        test=Return_Type::Bool(false);
                    }
                }
            },
            Err(_e) => {
                test=Return_Type::String(conerr::ConInterrupt.to_string());
            },
        }
        test
    }
    pub fn last(&self) -> Result<i32,MsfError> {
        let test;
        let mut body=Vec::new();
        self.serialize_without_arg("session.ring_last",&mut body);
        let con=connect(self.client.url,body);
        match con {
            Ok(val) => {
                if val.get("seq") == None {
                    let ret:MsfError=from_value(val).unwrap();
                    test=Return_Type::MsfErr(ret);
                } else {
                    test=Return_Type::Int(val.get("seq").unwrap().as_i64().unwrap());
                }
            },
            Err(_e) => {
                test=Return_Type::String(conerr::ConInterrupt.to_string());
            },
        }
        test
    }
    pub fn put(&self,data:String) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize_with_arg("session.ring_put",data,&mut body);
        let con=connect(self.client.url,body);
        match con {
            Ok(val) => {
                if val.get("write_count") == None {
                    let ret:MsfError=from_value(val).unwrap();
                    test=Return_Type::MsfErr(ret);
                } else {
                    test=Return_Type::Int(val.get("write_count").unwrap().as_i64().unwrap());
                }
            },
            Err(_e) => {
                test=Return_Type::String(conerr::ConInterrupt.to_string());
            },
        }
        test
    }
}
