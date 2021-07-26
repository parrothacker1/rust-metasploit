use std::fmt::{Result,Display,Debug};
use ureq;
pub type ConnectionError=ureq::Error;
#[derive(Debug)]
pub struct MsfError {
    error:bool,
    error_class:String,
    error_message:String,
}
pub fn connectionerr(socket:String) {
	panic!("Connection to Metasploit RPC Server hosted at {} failed",socket);
}
