//! This is a metasploit RPC library which makes a HTTP Connection with the Metasploit RCP Server.
//! All the modules and functions are written on the basis of Metasploit RPC Guide.
//! # Installation
//!
//! Rust-metasploit comes with two features. One is blocking and second is async.Deafult is set to be blocking
//! Async features needs to be used when you are dealing with tokio crate too.
//! For installation , add the follwing line in your Cargo.toml under dependencies
//!
//! ```
//! [dependencies]
//! rust-metasploit={ version="1.2.0",features=["blocking"] }
//! ```
//! # Rust-Metasploit
//! As said earlier,this library is usedto connect with the msfrpcd server.
//!
//! ## Example
//! ```
//! use metasploit::client::Client;
//! fn main() {
//!     let client=Client::new("127.0.0.1",55552,"msf","password",true);
//!     println!("{}",client.gettoken());
//! }
//! ```
//!
//! The above one is a simple example code where an connection is made with RPC Server and the token is printed
//! There is also another example such as this one below
//! 
//! ## Example
//! ```
//! use metasploit::client::Client;
//! use metasploit::msf::auth;
//! use serde::Deserializeas des;
//!
//! [#derive(des,Debug)]
//! struct AddToken {
//!     pub result:String,
//! }
//! fn main() {
//!     let client=Client::new("127.0.0.1",55552,"msf","password",true);
//!     let token:AddToken=auth::add_token(client,"newtoken");
//!     println!("{}",token.result);
//! }
//! ```
//!
//! Here the data type of token variable can be a custom created struct which can phrase the response from the msfrpcd server.
//! The data type struct should only have feilds that are defined by the server.Any other exception will lead to an error.
//! Some of the data types are defined in the previous versions of rust-metaslpoit (They are not up-to-date with the latest release).
//! For more information on the data types , you can refer the metasploit documentation.
//!
pub mod client;
pub mod msf;
pub mod error;
pub mod value;
