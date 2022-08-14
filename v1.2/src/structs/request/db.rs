use serde::Serialize as se;
use std::collections::HashMap;

#[derive(se)]
pub struct hosts(pub String,pub String);
#[derive(se)]
pub struct workspaces(pub String,pub String);
#[derive(se)]
pub struct current_workspace(pub String,pub String);
#[derive(se)]
pub struct gsda_workspace(pub String,pub String,pub String);
#[derive(se)]
pub struct nc(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct grd_host(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct disconnect(pub String,pub String);
#[derive(se)]
pub struct status(pub String,pub String);
#[derive(se)]
pub struct connect(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct grd_service(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct grd_vuln(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct grd_note(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct grd_client(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct get_ref(pub String,pub String,pub String);
#[derive(se)]
pub struct report_event(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct report_loot(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct driver(pub String,pub String,pub HashMap<String,String>);
#[derive(se)]
pub struct import_db(pub String,pub String,pub String);
