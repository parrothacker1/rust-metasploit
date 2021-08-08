#![allow(non_camel_case_types)]
use serde::Serialize as se;

#[derive(se)]
pub struct create(pub String,pub String);
#[derive(se)]
pub struct destroy(pub String,pub String,pub String);
#[derive(se)]
pub struct list(pub String,pub String);
#[derive(se)]
pub struct write(pub String,pub String,pub String,pub String);
#[derive(se)]
pub struct read(pub String,pub String,pub String);
#[derive(se)]
pub struct session_detach(pub String,pub String,pub String);
#[derive(se)]
pub struct session_kill(pub String,pub String,pub String);
#[derive(se)]
pub struct tabs(pub String,pub String,pub String,pub String);
