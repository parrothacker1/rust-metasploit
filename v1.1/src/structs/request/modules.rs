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
#[derive(se)]
pub struct options(pub String,pub String,pub String,pub String);
#[derive(se)]
pub struct compactible(pub String,pub String,pub String);
#[derive(se)]
pub struct compactible_tp(pub String,pub String,pub String,pub i32);
#[derive(se)]
pub struct search(pub String,pub String,pub String);
#[derive(se)]
pub struct check(pub String,pub String,pub String,pub String,pub HashMap<String,String>);
