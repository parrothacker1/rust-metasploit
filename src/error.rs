use std::fmt::{Result,Display,Formatter,Debug};
use reqwest;
use serde::Deserialize as des;
use snafu::Snafu;
pub type ConnectionError=reqwest::Error;
#[derive(des)]
pub struct MsfError {
    error:bool,
    error_class:String,
    error_message:String,
}
impl Display for MsfError {
	fn fmt(&self,f: &mut Formatter) -> Result {
		write!(f,"Error Message {}",self.error_message)
	}
}
impl Debug for MsfError {
	fn fmt(&self,f:&mut Formatter) -> Result {
		write!(f,"{:?}",self)
	}
}
	
