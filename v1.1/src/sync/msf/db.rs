//! A module to manage backend connectivity.
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
use structs::{request as req,response as res};
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,{decode::Error as derror,from_read}};
use crate::{error::MsfError,client::Client};
use std::collections::HashMap;

pub fn hosts(client:Client,xopts:Option<HashMap<String,String>>) -> Result<bool,MsfError> {
	let mut test:Result<bool,MsfError>=Ok(true);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let opts:HashMap<String,String>;
    if xopts==None {
        opts=HashMap::new();
    } else {
        opts=xopts.unwrap();
    }
    let byte=req::db::hosts("db.hosts".to_string(),opts);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::>
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        }
    }
}
pub fn services(client:Client) {

}
pub fn
