#![cfg_attr(docsrs, feature(doc_cfg))]
//! The module which contain all the necessary async modules to communicate with RPC Server
pub mod auth;
pub mod console;
pub mod core;
pub mod jobs;
pub mod modules;
pub mod plugins;
pub mod sessions;
#[cfg(any(feature="blocking",doc))]
#[cfg_attr(docsrs,doc(cfg(feature = "blocking")))]
pub mod blocking;
