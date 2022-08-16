use crate::client::Client;
use crate::error::{Error as E,MsfError};
#[path="../../connect.rs"] mod connect;
use serde::{ser::Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};

pub fn custom<T:DOwned,S:Serialize>(client:Client,byte:S) -> Result<T,E> {
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    byte.serialize(&mut serializer).unwrap();
    let con=connect::connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    match con {
        Ok(_) => {
            let ret:Result<T,derror>=from_read(new_buf.as_slice());
            match ret {
                Ok(val) => {
                    Ok(val)
                },
                Err(_) => {
                    let ret2:Result<MsfError,derror>=from_read(new_buf.as_slice());
                    match ret2 {
                        Ok(e) => {
                            Err(E::MsfError(e))
                        },
                        Err(e) => {
                            Err(E::DError(e))
                        },
                    }
                },
            }
        },
        Err(e) => {
            Err(E::ConnectionError(e))
        },
    }
}
