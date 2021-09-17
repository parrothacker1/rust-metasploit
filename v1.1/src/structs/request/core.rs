#![allow(non_camel_case_types)]
use serde::Serialize as se;

#[derive(se)]
pub struct addmodpath(pub String,pub String,pub String);
#[derive(se)]
pub struct modulestat(pub String,pub String);
#[derive(se)]
pub struct reloadmod(pub String,pub String);
#[derive(se)]
pub struct save(pub String,pub String);
#[derive(se)]
pub struct setg(pub String,pub String,pub String,pub String);
#[derive(se)]
pub struct unsetg(pub String,pub String,pub String);
#[derive(se)]
pub struct threadlist(pub String,pub String);
#[derive(se)]
pub struct threadkill(pub String,pub String,pub i32);
#[derive(se)]
pub struct version(pub String,pub String);
#[derive(se)]
pub struct stop(pub String,pub String);