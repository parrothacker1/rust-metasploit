//! A module to handle the jobs in Metasploit
#[path="../../structs/mod.rs"] mod structs;
#[path="../../error.rs"] mod error;
#[path="../../connect.rs"] mod connect;
use crate::client::Client;
use connect::connect;
use error::MsfError;
use std::collections::HashMap;
use rmp_serde::{Serializer,Deserializer,decode::{Error as derror,from_read}};
use serde::{Serialize,Deserialize};
use structs::{request as req,response as res};

/// To list all the currently running jobs
///
/// ## Example
/// ```
/// jobs::list(client.clone()).await.unwrap(); // {"1":"<jobname>"}
/// ```
pub async fn list(client:Client) -> Result<HashMap<String,String>,MsfError> {
    let mut test:Result<HashMap<String,String>,MsfError>=Ok(HashMap::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::jobs::list("job.list".to_string(),client.token.unwrap());
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<HashMap<String,String>,derror>=Deserialize::deserialize(&mut de);
            if let Ok(ref val) = de_ret {
                test=Ok(val.clone());
            };
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
/// To get information about the specified job
///
/// ## Example
/// ```
/// jobs::info(client.clone(),"1").await.unwrap(); // response::jobs::info {}
/// ```
pub async fn info(client:Client,jobidstr:&str) -> Result<res::jobs::info,MsfError> {
    let jobid:String=jobidstr.to_string();
    let mut test:Result<res::jobs::info,MsfError>=Ok(res::jobs::info{
        jid:0,
        start_time:0,
        name:String::new(),
        uripath:String::new(),
        datastore:res::jobs::Data {
            EnableContextEncoding:true,
            DisablePayloadHandler:true,
            SSL:true,
            SSLVersion:String::new(),
            PAYLOAD:String::new(),
        },
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::jobs::info("job.info".to_string(),client.token.unwrap(),jobid);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::jobs::info,derror>=Deserialize::deserialize(&mut de);
            if let Ok(ref val) = de_ret {
                test=Ok(val.clone());
            };
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
/// To stop a specified job
///
/// ## Example
/// ```
/// jobs::stop(client.clone(),"1").await.unwrap(); // true
/// ```
pub async fn stop(client:Client,jobidstr:&str) -> Result<bool,MsfError> {
    let jobid:String=jobidstr.to_string();
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::jobs::stop("job.stop".to_string(),client.token.unwrap(),jobid);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::jobs::stop,derror>=Deserialize::deserialize(&mut de);
            if let Ok(ref val) = de_ret {
                if val.result=="success".to_string() {
                    test=Ok(true);
                } else {
                    test=Ok(false);
                }
            };
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}