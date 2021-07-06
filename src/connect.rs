use reqwest::{Response,Client,header,Error};
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::ReadReader};

pub fn conn(url:String,body:Vec<u8>) {
    let mut request:Result<Response,Error>;
    let mut hd=header::HeaderMap::new();
    hd.insert(header::CONTENT_TYPE,header::HeaderValue::from_static("binary/message-pack"));
    let client:Result<Client,Error>=Client::builder().default_headers(hd).build();
    match client {
        Ok(client) => {
            request=client.post(&url).body(body).send();
            match request {
                Ok(req) => {
                    print!("Done")
                },
                Err(e) => {
                    print!("error");
                },
            }
        },
        Err(e) => {
            println!("error");
        },
    }
}
