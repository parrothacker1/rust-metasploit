#![allow(non_camel_case_types)]
use std::fmt::Debug;
use ureq;
use snafu::Snafu;
pub type ConnectionError=ureq::Error;
#[derive(Debug,Snafu)]
pub enum conerr {
	#[snafu(display("Couldn't cannot to Metasploit RPC Server at {}",socket))]
	ConnectionNotPossible { socket:String },
	#[snafu(display("Connection Interrupted while communicating"))]
	ConInterrupt,
    #[snafu(display("Authentication failed for user:{}",user))]
    Authfail { user:String },
}
