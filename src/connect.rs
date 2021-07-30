use ureq::{Agent,AgentBuilder};
use std::time::Duration;
use rmp_serde::{encode::to_vec,decode::from_read};
use serde_json::Value;
use std::io::Read;
#[path="./error.rs"] mod error;
use error::ConnectionError as conerr;

pub enum Parse_Type {
    Bool(bool),
    String(String),
    Int(i32),
}

pub fn connect(url:String,body:Vec<Parse_Type>) -> Result<Value,conerr> {
    let body_con=to_vec(&body);
    let agent:Agent=AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
	let reader=agent.post(&url)
		.set("Content-type","binary/message-pack")
		.send_bytes(body_con)?
		.into_reader();
    let test:Value=from_read(reader).unwrap();
    Ok(test)
}
