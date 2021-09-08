//! This module is to handle all error responses from RPC Server and to handle Connection Error.
#![allow(dead_code)]
use std::fmt::{Result,Display,Formatter,Debug};
use reqwest;
use std::error::Error;
use serde::Deserialize as des;

/// This is to handle connection errors.For more information refer [reqwest::Error](https://docs.rs/reqwest/0.7.2/reqwest/struct.Error.html)
pub type ConnectionError=reqwest::Error;

/// Struct to handle RPC error Responses
#[derive(des,Debug)]
/// ```
/// pub struct MsfError {/*... */}
/// ```
/// Struct to handle error responses from RPC Server
pub struct MsfError {
    /// Boolean value for error
    pub error:bool,
    /// Class of error
    pub error_class:String,
    /// Error description
    pub error_string:String,
    /// Error Message
    pub error_message:String,
    /// Error Backrrace
    pub error_backtrace:Vec<String>,
}
impl Error for MsfError {}

impl Display for MsfError {
	fn fmt(&self,f: &mut Formatter) -> Result {
		write!(f,"Error Message {}",self.error_message)
	}
}
