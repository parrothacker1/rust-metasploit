use std::fmt::{Result,Display,Debug};
use reqwest;
pub type ConnectionError=reqwest::error;
pub struct MsfError;
