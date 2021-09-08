#![allow(non_camel_case_types)]
use serde::Serialize as se;

#[derive(se)]
pub struct list(pub String,pub String);
#[derive(se)]
pub struct stop(pub String,pub String,pub String);
#[derive(se)]
pub struct shell_read_with_pointer(pub String,pub String,pub String,pub i32);
#[derive(se)]
pub struct shell_read(pub String,pub String,pub String);
#[derive(se)]
pub struct shell_write(pub String,pub String,pub String,pub String);
#[derive(se)]
pub struct meterpreter_with_one(pub String,pub String,pub String);
#[derive(se)]
pub struct meterpreter_with_two(pub String,pub String,pub String,pub String);
#[derive(se)]
pub struct shell_upgrade(pub String,pub String,pub String,pub String,pub i32);
#[derive(se)]
pub struct ring_with_arg(pub String,pub String,pub String,pub String);
#[derive(se)]
pub struct ring_without_arg(pub String,pub String,pub String);
