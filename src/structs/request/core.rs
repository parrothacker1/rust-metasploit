use serde::Serialize as se;

#[derive(se)]
pub struct addmodpath(pub String,pub String,pub String);
#[derive(se)]
pub struct modulestat(pub String,pub String);
#[derive(se)]
pub struct reloadmod(pub String,pub String);
#[derive(se)]
pub struct save(pub String,pub String);

