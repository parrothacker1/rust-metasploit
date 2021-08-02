#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use std::collections::HashMap;
use error::conerr;
use common::{MsfError,ReturnValue as Return_Type,modulelist,info,moduleoption,compactiblepayload,compactiblesessions};
use serde_json::from_value;
use serde::Serialize as se;
use rmp_serde::Serializer;

pub struct compactible {
    pub name:String,
    pub client:Client,
}
pub struct list {
    pub client:Client,
}
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
#[derive(se)]
struct moduleliststruct(String,String);
impl list {
    pub fn new(client:Client) -> Self {
        list {
            client:client,
        }
    }
    fn serialize(self,method:&str,body:&mut Vec<u8>) {
		let mut serializer=Serializer::new(&mut body);
		let byte=moduleliststruct(method.to_string(),self.client.token.unwrap());
		byte.serialize(&mut serializer).unwrap();
	}
    pub fn exploits(self) -> Return_Type {
        let mut body=Vec::new();
        let test;
        self.serialize("module.exploits",&mut body);
        let con=connect::connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<modulelist,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::ArrayStr(val.modules);
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
    pub fn auxiliary(self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize("module.auxiliary",&mut body);
        let con=connect::connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<modulelist,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::ArrayStr(val.modules);
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
    pub fn post(self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize("module.post",&mut body);
        let con=connect::connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<modulelist,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::ArrayStr(val.modules);
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
    pub fn payloads(self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize("module.payloads",&mut body);
        let con=connect::connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<modulelist,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::ArrayStr(val.modules);
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
    pub fn encoders(self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize("module.encoders",&mut body);
        let con=connect::connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<modulelist,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::ArrayStr(val.modules);
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
    pub fn nops(self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize("module.nops",&mut body);
        let con=connect::connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<modulelist,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::ArrayStr(val.modules);
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
}
#[derive(se)]
struct moduleinfo(String,String,String,String);
pub fn info(client:Client,moduletype:String,modulename:String) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=moduleinfo("module.info".to_string(),client.token.unwrap(),moduletype,modulename);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
		Ok(val) => {
			let ret:Result<info,serde_json::Error>=from_value(val);
			match ret {
				Ok(val) => {
					test=Return_Type::ModuleInfo(val);
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
pub fn option(client:Client,moduletype:String,modulename:String) -> Return_Type {
	let test;
	let mut body=Vec::new();
	let mut serializer=Serializer::new(&mut body);
	let byte=moduleinfo("module.options".to_string(),client.token.unwrap(),moduletype,modulename);
	byte.serialize(&mut serializer);
	let con=connect::connect(client.url,body);
	match con {
		Ok(val) => {
			let ret:Result<HashMap<String,moduleoption>,serde_json::Error>=from_value(val);
			match ret {
				Ok(val) => {
					test=Return_Type::ModuleOption(val);
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
struct compactiblestruct(String,String,String);
#[derive(se)]
struct targetcompactiblestruct(String,String,String,i32);
impl compactible {
    pub fn new(modulename:String,client:Client) -> Self {
        compactible {
            name:modulename,
            client:client,
        }
    }
    fn serialize(self,method:&str,body:&mut Vec<u8>) {
		let mut serializer=Serializer::new(&mut body);
		let byte=compactiblestruct(method.to_string(),self.client.token.unwrap(),self.name);
		byte.serialize(&mut serializer).unwrap();
	}
    pub fn payload(self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize("module.compactible_payloads",&mut body);
        let con=connect::connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<compactiblepayload,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::ArrayStr(val.payloads);
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
    pub fn target_payload(self,targetindx:i32) -> Return_Type {
        let test;
        let mut body=Vec::new();
        let mut serializer=Serializer::new(&mut body);
        let byte=targetcompactiblestruct("module.target_compactible_payloads".to_string(),self.client.token.unwrap(),self.name,targetindx);
        byte.serialize(&mut serializer).unwrap();
        let con=connect::connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<compactiblepayload,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::ArrayStr(val.payloads);
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
    pub fn sessions(self) -> Return_Type {
        let test;
        let mut body=Vec::new();
        self.serialize("module.compactible_sessions",&mut body);
        let con=connect::connect(self.client.url,body);
        match con {
			Ok(val) => {
				let ret:Result<compactiblesessions,serde_json::Error>=from_value(val);
				match ret {
					Ok(val) => {
						test=Return_Type::ArrayInt(val.sessions);
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

}
#[derive(se)]
struct modulencode(String,String,String,String,HashMap<String,String>);
pub fn encoder(client:Client,data:String,encodermodule:String,options:HashMap<String,String>) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=modulencode("module.encode".to_string(),client.token.unwrap(),data,encodermodule,options);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if val.get("encoded")==None {
                let ret:MsfError=from_value(val).unwrap();
                test=Return_Type::MsfErr(ret);
            } else {
                test=Return_Type::String(val.get("encoded").unwrap().as_str().unwrap().to_string());
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
#[derive(se)]
struct moduleexecute(String,String,String,String,HashMap<String,String>);
pub fn execute(client:Client,moduletype:String,modulename:String,options:HashMap<String,String>) -> Return_Type {
    let test;
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=moduleexecute("module.execute".to_string(),client.token.unwrap(),moduletype,modulename,options);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body);
    match con {
        Ok(val) => {
            if moduletype != "payload".to_string() {
                if val.get("payload") == None {
                    let ret:MsfError=from_value(val).unwrap();
                    test=Return_Type::MsfErr(ret);
                } else {
                    test=Return_Type::String(val.get("payload").unwrap().as_str().unwrap().to_string());
                }
            } else {
                if val.get("job_id") == None {
                    let ret:MsfError=from_value(val).unwrap();
                    test=Return_Type::MsfErr(ret);
                } else {
                    test=Return_Type::Int(val.get("job_id").unwrap().as_i64().unwrap());
                }
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
