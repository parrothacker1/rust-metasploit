//! A module to manage backend connectivity.
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use structs::{request as req,response as res};
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,{decode::Error as derror,from_read}};
use crate::{error::MsfError,client::Client};

/*pub fn hosts(client:Client) {
	println!("Hello");
}
pub fn services(client:Client) {

}
pub fn */
