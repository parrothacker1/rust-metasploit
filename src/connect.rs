use reqwest::{header,blocking::Client};
use std::time::Duration;


//use std::io::Read;
#[path="./error.rs"] mod error;
use error::{ConnectionError as conerr};

pub fn connect(url:String,body:Vec<u8>,buf:&mut Vec<u8>) -> Result<(),conerr> {
	let mut header=header::HeaderMap::new();
	header.insert(header::CONTENT_TYPE,header::HeaderValue::from_static("binary/message-pack"));
    let client=Client::builder()
		.default_headers(header)
		.timeout(Duration::from_secs(10))
		.danger_accept_invalid_certs(true)
		.build()
		.unwrap();
	let mut reader=client.post(url).body(body).send()?;
	reader.copy_to(buf).unwrap();
    Ok(())
}
