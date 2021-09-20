//! A module to manage backend connectivity.
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use structs::{request as req,response as res};
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,{decode::Error as derror,from_read}};
use crate::{error::MsfError,client::Client};

pub fn hosts(client:Client) -> Result<bool,MsfError> {
	let mut test:Result<bool,MsfError>=Ok(true);
    let mut body=Vec::new();
    let mut buf=vec![];
    let byte=req::
}
pub fn services(client:Client) {

}
pub fn
