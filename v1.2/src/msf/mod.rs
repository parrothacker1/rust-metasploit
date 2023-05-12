#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unused_attributes)]
//! The module which contain all the necessary async modules to communicate with RPC Server

//#[cfg(all(feature="async",feature="blocking"))]
//compile_error!("Feature \"async\" and feature \"blocking\" cannot be used at once. Disable default features to use \"async\" feature");

#[cfg_attr(feature="async",path="async/auth.rs")]
#[cfg_attr(feature="blocking",path="blocking/auth.rs")]
pub mod auth;

#[cfg_attr(feature="async",path="async/console.rs")]
#[cfg_attr(feature="blocking",path="blocking/console.rs")]
pub mod console;

#[cfg_attr(feature="async",path="async/core.rs")]
#[cfg_attr(feature="blocking",path="blocking/core.rs")]
pub mod core;

#[cfg_attr(feature="async",path="async/jobs.rs")]
#[cfg_attr(feature="blocking",path="blocking/jobs.rs")]
pub mod jobs;

#[cfg_attr(feature="async",path="async/modules.rs")]
#[cfg_attr(feature="blocking",path="blocking/modules.rs")]
pub mod modules;

#[cfg_attr(feature="async",path="async/plugins.rs")]
#[cfg_attr(feature="blocking",path="blocking/plugins.rs")]
pub mod plugins;

#[cfg_attr(feature="async",path="async/sessions.rs")]
#[cfg_attr(feature="blocking",path="blocking/sessions.rs")]
pub mod sessions;

#[cfg_attr(feature="async",path="async/db.rs")]
#[cfg_attr(feature="blocking",path="blocking/db.rs")]
pub mod db;
