use reqwest::{Response,Client,header,Error};
use rmp_serde::{Serializer,Deserializer,decode::ReadReader};
mod structs;
use structs::auth::login;
pub fn auth(url:String,username:String,password:String) -> String {
    let body:Vec<u8>=vec![];
    let mut request:Result<Response,Error>;
    let mut hd=header::HeaderMap::new();
    hd.insert(header::CONTENT_TYPE,header::HeaderValue::from_static("binary/message-pack"));
    let client:Result<Client,Error>=Client::builder().default_headers(hd).build();
    let test:String=String::new();
    match client {
        Ok(client) => {
            request=client.post(&url).body(body).send();
            match request {
                Ok(req) => {
                    print!("Done")
                },
                Err(_e) => {
                    print!("error");
                },
            }
        },
        Err(_e) => {
            println!("error");
        },
    };
    test
}
