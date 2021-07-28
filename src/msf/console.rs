#[path="../structs/mod.rs"] mod structs;
#[path="../connect.rs"] mod connect;
#[path="../error.rs"] mod error;
#[path="./common.rs"] mod common;
use common::{MsfError,Return_Type};
use error::conerr;
use structs::console;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
pub fn create(client:Client) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
        Ok(val) => {
            if val.get("result").unwrap().as_str().unwrap()=="success" {
                test=Return_Type::Bool(true);
            } else {
                let ret=MsfError {
                    error:val.get("error").unwrap().as_bool().unwrap(),
                    error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
                    error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
                };
                test=Return_Type::MsfErr(ret);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn destroy(client:Client,consoleid:String) -> Return_Type {
    let test;
    let conn=connect::connect(client.url);
    match conn {
        Ok(val) => {
            if val.get("result").unwrap().as_str().unwrap()=="success" {
                test=Return_Type::Bool(true);
            } else {
                let ret=MsfError {
                    error:val.get("error").unwrap().as_bool().unwrap(),
                    error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
                    error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
                };
                test=Return_Type::MsfErr(ret);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn write(client:Client,consoleid:String,data:String) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
        Ok(val) => {
            if val.get("wrote") == None {
                let ret=MsfError {
                    error:val.get("error").unwrap().as_bool().unwrap(),
                    error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
                    error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
                };
                test=Return_Type::MsfErr(ret);
            } else {
                let wrote_string:String=val.get("wrote").unwrap().as_str().unwrap().to_string();
                let wrote:i32=wrote_string.parse().unwrap();
                test=Return_Type::Int(wrote);
            }
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn read(client:Client,consoleid:String) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
        Ok(val) => {
            if val.get("data")==None {
                let ret=MsfError {
					error:val.get("error").unwrap().as_bool().unwrap(),
					error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
					error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
				};
				test=Return_Type::MsfErr(ret);
			} else {
				let ret=console::read {
					data:val.get("data").unwrap().as_str().unwrap().to_string(),
					prompt:val.get("prompt").unwrap().as_str().unwrap().to_string(),
					busy:val.get("busy").unwrap().as_bool().unwrap(),
				};
			};
        },
        Err(_e) => {
            test=Return_Type::String(conerr::ConInterrupt.to_string());
        },
    }
    test
}
pub fn session_detach(client:Client,consoleid:String) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				test=Return_Type::Bool(true);
			} else {
				let ret=MsfError {
					error:val.get("error").unwrap().as_bool().unwrap(),
					error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
					error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
				};
				test=Return_Type::MsfErr(ret);
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
pub fn session_kill(client:Client,consoleid:String) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
		Ok(val) => {
			if val.get("result").unwrap().as_str().unwrap()=="success" {
				test=Return_Type::Bool(true);
			} else {
				let ret=MsfError {
					error:val.get("error").unwrap().as_bool().unwrap(),
					error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
					error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
				};
				test=Return_Type::MsfErr(ret);
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
pub fn tabs(client:Client,consoleid:String,inputline:String) -> Return_Type {
    let test;
    let con=connect::connect(client.url);
    match con {
		Ok(val) => {
			if val.get("tabs")==None {
				let ret=MsfError {
					error:val.get("error").unwrap().as_bool().unwrap(),
					error_class:val.get("error_class").unwrap().as_str().unwrap().to_string(),
					error_message:val.get("error_message").unwrap().as_str().unwrap().to_string(),
				};
				test=Return_Type::MsfErr(ret);
			} else {
				let ret=val.get("tabs").unwrap().as_array().unwrap().to_vec();
				test=Return_Type::Array(ret);
			}
		},
		Err(_e) => {
			test=Return_Type::String(conerr::ConInterrupt.to_string());
		},
	}
    test
}
