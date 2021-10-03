//! This is a metasploit RPC library which makes a HTTP Connection with the Metasploit RCP Server.
//! All the modules and functions are written on the basis of Metasploit RPC Guide.
//! ## Example
//! ```
//! use metasploit::client::Client;
//! let client=Client::new("127.0.0.1",55552,"msf","password",true);
//! println!("{}",client.gettoken());
//! ```
//! The above one is a simple example code where an connection is made with RPC Server and the token is printed
//!
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, deny(rustdoc::broken_intra_doc_links))]
pub mod client;
pub mod error;
pub mod value;
#[path="structs/response/mod.rs"] pub mod response;
/// Async functions are available in "async" features
/*#[cfg(any(feature="async",doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]*/
pub mod r#async;
/// Sync functions are available in "sync" features
/*#[cfg(any(features="sync",doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]*/
pub mod sync;
