use ureq::{Agent,AgentBuilder};
use std::time::Duration;
use rmp_serde::decode::from_read;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Read;
#[path="./error.rs"] mod error;
use error::{ConnectionError as conerr,MsfError as msferr};

pub fn connect(url:String,) -> Result<Value,conerr> {
    let agent:Agent=AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
	let reader=agent.post(&url)
		.set("Content-type","binary/message-pack")
		.call()?
		.into_reader();
    let test:Value=from_read(reader).unwrap();
    Ok(test)
}
