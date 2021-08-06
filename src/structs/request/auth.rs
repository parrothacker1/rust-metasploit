use serde::Serialize as se;

#[derive(se)]
pub struct login(pub String,pub String,pub String);
#[derive(se)]
pub struct logout(pub String,pub String,pub String);
#[derive(se)]
pub struct tokenadd(pub String,pub String,pub String);
#[derive(se)]
pub struct tokengen(pub String,pub String);
#[derive(se)]
pub struct tokenlist(pub String,pub String);
#[derive(se)]
pub struct tokenrem(pub String,pub String,pub String);
