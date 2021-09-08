#![allow(non_camel_case_types)]
use serde::Serialize as se;

#[derive(se)]
pub struct list(pub String,pub String);
#[derive(se)]
pub struct info(pub String,pub String,pub String);
#[derive(se)]
pub struct stop(pub String,pub String,pub String);