#[path="../../connect.rs"] mod connect;
#[path="../../structs/mod.rs"] mod structs;
use crate::error::{MsfError,Error as E};
use structs::request as req;
use crate::client::Client;
use serde::{Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};

pub fn hosts<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::hosts("db.hosts".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body)

}
pub fn get_host() {}
pub fn report_host() {}
pub fn del_host() {}

pub fn services() {}
pub fn report_service() {}
pub fn get_service() {}
pub fn del_service() {}

pub fn vulns() {}
pub fn del_vuln() {}
pub fn report_vuln() {}
pub fn get_vuln() {}

pub fn workspaces(client:Client) {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::workspaces("db.workspaces".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn current_workspace(client:Client) {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::current_workspacs("db.current_workspace".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn get_workspace(client:Client,workspace:str) {}
pub fn set_workspace(client:Client,workspace:str) {}
pub fn del_workspace(client:Client,workspace:str) {}
pub fn add_workspace(client:Client,workspace:str) {}

pub fn get_note() {}
pub fn report_note() {}
pub fn notes(client,{}) {}
pub fn del_note() {}

pub fn get_client() {}
pub fn clients() {}
pub fn del_client() {}
pub fn report_client() {}

pub fn get_ref() {}

pub fn events() {}
pub fn report_event() {}

pub fn report_loot() {}
pub fn loots() {}

pub fn report_cred() {}
pub fn creds() {}

pub fn import_data() {}

pub fn driver() {}

pub fn connect() {}

pub fn status() {}

pub fn disconnect() {}

fn r#return<T:DOwned>(url:String,body:Vec<u8>) -> Result<T,E> {
    let mut buf=vec![];
    let con=connect::connect(url,body,&mut buf);
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
    
