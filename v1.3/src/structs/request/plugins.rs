#![allow(non_camel_case_types)]
use std::collections::HashMap;
use serde::Serialize as se;

#[derive(se)]
pub struct load(pub String,pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct unload(pub String,pub String,pub String);
#[derive(se)]
pub struct loaded(pub String,pub String);