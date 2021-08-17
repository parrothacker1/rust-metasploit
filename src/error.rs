#![allow(dead_code)]
use std::fmt::{Result,Display,Formatter,Debug};
use reqwest;
use std::error::Error;
use serde::Deserialize as des;

pub type ConnectionError=reqwest::Error;
#[derive(des,Debug)]
pub struct MsfError {
    error:bool,
    error_class:String,
    error_string:String,
    error_message:String,
}
impl Error for MsfError {}
impl Display for MsfError {
	fn fmt(&self,f: &mut Formatter) -> Result {
		write!(f,"Error Message {}",self.error_message)
	}
}
	
