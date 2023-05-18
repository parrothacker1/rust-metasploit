#![allow(dead_code)]
use std::fmt::{Result,Display,Formatter,Debug};
use reqwest;
use std::env::var;
use serde::Deserialize as des;
use rmp_serde::decode;
pub type ConnectionError=reqwest::Error;

pub type DError=decode::Error;

#[derive(des,Debug)]
pub struct MsfError {
    pub error:bool,
    pub error_class:String,
    pub error_string:String,
    pub error_message:String,
    pub error_backtrace:Vec<String>,
}

impl std::error::Error for MsfError {}

impl Display for MsfError {
	fn fmt(&self,f: &mut Formatter) -> Result {
        let err:String;
        match var("RUST_BACKTRACE") {
            Ok(val) => {
                if val=="1".to_string() {
                    err=format!("({},{})",self.error_message,self.error_class).to_string()
                } else if val=="full".to_string() {
                    err=format!("{:?}",self).to_string();
                } else {
                    err=format!("{}",self.error_message).to_string();
                }
            },
            Err(_) => {
                err=format!("{}",self.error_message);
            },
        }
		write!(f,"{}",err)
	}
}

#[derive(Debug)]
pub enum Error {
    ConnectionError(ConnectionError),
    DError(DError),
    MsfError(MsfError),
}

impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self,f:&mut Formatter) -> Result {
        match self {
            Error::ConnectionError(e) => Display::fmt(&e,f),
            Error::DError(e) =>  Display::fmt(&e,f),
            Error::MsfError(e) =>  Display::fmt(&e,f),
        }
    }
}

impl From<ConnectionError> for Error {
    fn from(e:ConnectionError) -> Error {
        Error::ConnectionError(e)
    }
}
impl From<DError> for Error {
    fn from(e:DError) -> Error {
        Error::DError(e)
    }
}
impl From<MsfError> for Error {
    fn from(e:MsfError) -> Error {
        Error::MsfError(e)
    }
}
