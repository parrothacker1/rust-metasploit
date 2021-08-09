#![allow(dead_code)]
use std::fmt::{Result,Display,Formatter,Debug};
use reqwest;
use serde::Deserialize as des;

pub type ConnectionError=reqwest::Error;
#[derive(des)]
pub struct MsfError {
    error:bool,
    error_class:String,
    error_string:String,
}
impl Display for MsfError {
	fn fmt(&self,f: &mut Formatter) -> Result {
		write!(f,"Error Message {}",self.error_class)
	}
}
impl Debug for MsfError {
	fn fmt(&self,f:&mut Formatter) -> Result {
		write!(f,"{:?}",self)
	}
}
	
