//! This is a metasploit RPC library which makes a HTTP Connection with the Metasploit RCP Server.
//! All the modules and functions are written on the basis of Metasploit RPC Guide.
//! ## Example
//! ```
//! use metasploit::client::Client;
//! fn main() {
//!     let client=Client::new("127.0.0.1",55552,"msf","password",true);
//!     println!("{}",client.gettoken());
//! }
//! ```
//! The above one is a simple example code where an connection is made with RPC Server and the token is printed
//!
pub mod client;
pub mod msf;
pub mod error;
pub mod value;
