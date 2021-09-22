#![allow(non_camel_case_types)]
use serde::Serialize as se;
use std::collections::HashMap;

#[derive(se)]
pub struct hosts(pub String,pub HashMap<String,String>);
