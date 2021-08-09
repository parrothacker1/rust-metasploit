#![allow(non_camel_case_types)]
use serde::Serialize as se;
use std::collections::HashMap;

#[derive(se)]
pub  struct list(pub String,pub String);
#[derive(se)]
pub struct info(pub String,pub String,pub String,pub String);
#[derive(se)]
pub struct encoder(pub String,pub String,pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct execute(pub String,pub String,pub String,pub String,pub HashMap<String,String>);