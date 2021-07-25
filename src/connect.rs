use ureq::{Agent,AgentBuilder};
use std::time::Duration;
use rmp_serde::{decode::ReadReader,Deserializer};
use serde::Deserialize;
use std::collections::HashMap;

#[path="./error.rs"] mod error;
use error::{ConnectionError as conerr,MsfError as msferr};

pub fn connect(url:String) -> Result<HashMap<String,String>,conerr> {
    let agent:Agent=AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let reader=agent.post(&url)
        .set("Content-type","binary/message-pack")
        .call()?
        .into_reader();
    let byte=vec![];
    reader.read_to_end(&mut byte);
    let de=Deserializer::new(byte.as_slice());
    let decoded:HashMap<String,String>=Deserialize::deserialize(&mut de).unwrap();
    Ok(decoded)
}
