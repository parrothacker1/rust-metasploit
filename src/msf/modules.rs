#![allow(non_camel_case_types)]
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
#[path="../connect.rs"] mod connect;
use std::collections::HashMap;
use error::conerr;
use common::{MsfError,ReturnValue as Return_Type,modulelist,info,moduleoption,compactiblepayload,compactiblesessions};
use serde_json::from_value;
use connect::Parse_Type as PType;

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
impl list {
    pub fn new(client:Client) -> Self {
        list {
            client:client,
        }
    }
    pub fn exploits(self) -> Return_Type {
        let test;
        let body=vec![PType::String("module.exploits".to_string()),PType::String(self.client.token.unwrap())];
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
        let body=vec![PType::String("module.auxiliary".to_string()),PType::String(self.client.token.unwrap())];
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
        let body=vec![PType::String("module.post".to_string()),PType::String(self.client.token.unwrap())];
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
        let body=vec![PType::String("module.payloads".to_string()),PType::String(self.client.token.unwrap())];
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
        let body=vec![PType::String("module.encoders".to_string()),PType::String(self.client.token.unwrap())];
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
        let body=vec![PType::String("module.nops".to_string()),PType::String(self.client.token.unwrap())];
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
pub fn info(client:Client,moduletype:String,modulename:String) -> Return_Type {
    let test;
    let body=vec![PType::String("module.info".to_string()),PType::String(client.token.unwrap()),PType::String(moduletype),PType::String(modulename)];
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
	let body=vec![PType::String("module.options".to_string()),PType::String(client.token.unwrap()),PType::String(moduletype),PType::String(modulename)];
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
impl compactible {
    pub fn new(modulename:String,client:Client) -> Self {
        compactible {
            name:modulename,
            client:client,
        }
    }
    pub fn payload(self) -> Return_Type {
        let test;
        let body=vec![PType::String("module.compactible_payloads".to_string()),PType::String(self.client.token.unwrap()),PType::String(self.name)];
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
        let body=vec![PType::String("module.target_compatible_payloads".to_string()),PType::String(self.client.token.unwrap()),PType::String(self.name),PType::Int(targetindx)];
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
        let body=vec![PType::String("module.compatible_sessions".to_string()),PType::String(self.client.token.unwrap()),PType::String(self.name)];
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
pub fn encoder(client:Client,data:String,encodermodule:String,options:HashMap<String,String>) -> Return_Type {
    let test;
    let body=vec![PType::String("module.encode".to_string()),PType::String(client.token.unwrap()),PType::String(data),PType::String(encodermodule),PType::HashMapStr(options)];
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
pub fn execute(client:Client,moduletype:String,modulename:String,options:HashMap<String,String>) -> Return_Type {
    let test;
    let body=vec![PType::String("module.execute".to_string()),PType::String(client.token.unwrap()),PType::String(moduletype),PType::String(modulename),PType::HashMapStr(options)];
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
